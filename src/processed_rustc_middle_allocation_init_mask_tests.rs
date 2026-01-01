// SRC: ../rust/compiler/rustc_middle/src/mir/interpret/allocation/init_mask/tests.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=uninit_mask | COMPLEXITY=15 | LINES=20 */
use super::*;
use crate::mir::interpret::alloc_range;

#[test]
fn uninit_mask() {
    let mut mask = InitMask::new(Size::from_bytes(500), false);
    assert!(!mask.get(Size::from_bytes(499)));
    mask.set_range(alloc_range(Size::from_bytes(499), Size::from_bytes(1)), true);
    assert!(mask.get(Size::from_bytes(499)));
    mask.set_range((100..256).into(), true);
    for i in 0..100 {
        assert!(!mask.get(Size::from_bytes(i)), "{i} should not be set");
    }
    for i in 100..256 {
        assert!(mask.get(Size::from_bytes(i)), "{i} should be set");
    }
    for i in 256..499 {
        assert!(!mask.get(Size::from_bytes(i)), "{i} should not be set");
    }
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=materialized_block_count | COMPLEXITY=9 | LINES=8 */

/// Returns the number of materialized blocks for this mask.
fn materialized_block_count(mask: &InitMask) -> usize {
    match mask.blocks {
        InitMaskBlocks::Lazy { .. } => 0,
        InitMaskBlocks::Materialized(ref blocks) => blocks.blocks.len(),
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=materialize_mask_within_range | COMPLEXITY=11 | LINES=19 */

#[test]
fn materialize_mask_within_range() {
    // To have spare bits, we use a mask size smaller than its block size of 64.
    let mut mask = InitMask::new(Size::from_bytes(16), false);
    assert_eq!(materialized_block_count(&mask), 0);

    // Forces materialization, but doesn't require growth. This is case #1 documented in the
    // `set_range` method.
    mask.set_range((8..16).into(), true);
    assert_eq!(materialized_block_count(&mask), 1);

    for i in 0..8 {
        assert!(!mask.get(Size::from_bytes(i)), "{i} should not be set");
    }
    for i in 8..16 {
        assert!(mask.get(Size::from_bytes(i)), "{i} should be set");
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=grow_within_unused_bits_with_full_overwrite | COMPLEXITY=11 | LINES=20 */

#[test]
fn grow_within_unused_bits_with_full_overwrite() {
    // To have spare bits, we use a mask size smaller than its block size of 64.
    let mut mask = InitMask::new(Size::from_bytes(16), true);
    for i in 0..16 {
        assert!(mask.get(Size::from_bytes(i)), "{i} should be set");
    }

    // Grow without requiring an additional block. Full overwrite.
    // This can be fully handled without materialization.
    let range = (0..32).into();
    mask.set_range(range, true);

    for i in 0..32 {
        assert!(mask.get(Size::from_bytes(i)), "{i} should be set");
    }

    assert_eq!(materialized_block_count(&mask), 0);
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=grow_same_state_within_unused_bits | COMPLEXITY=16 | LINES=28 */

// This test checks that an initmask's spare capacity is correctly used when growing within block
// capacity. This can be fully handled without materialization.
#[test]
fn grow_same_state_within_unused_bits() {
    // To have spare bits, we use a mask size smaller than its block size of 64.
    let mut mask = InitMask::new(Size::from_bytes(16), true);
    for i in 0..16 {
        assert!(mask.get(Size::from_bytes(i)), "{i} should be set");
    }

    // Grow without requiring an additional block. The gap between the current length and the
    // range's beginning should be set to the same value as the range.
    let range = (24..32).into();
    mask.set_range(range, true);

    // We want to make sure the unused bits in the first block are correct
    for i in 16..24 {
        assert!(mask.get(Size::from_bytes(i)), "{i} should be set");
    }

    for i in 24..32 {
        assert!(mask.get(Size::from_bytes(i)), "{i} should be set");
    }

    assert_eq!(1, mask.range_as_init_chunks((0..32).into()).count());
    assert_eq!(materialized_block_count(&mask), 0);
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=grow_mixed_state_within_unused_bits | COMPLEXITY=16 | LINES=31 */

// This is the same test as `grow_same_state_within_unused_bits` but with both init and uninit
// states: this forces materialization; otherwise the mask could stay lazy even when needing to
// grow.
#[test]
fn grow_mixed_state_within_unused_bits() {
    // To have spare bits, we use a mask size smaller than its block size of 64.
    let mut mask = InitMask::new(Size::from_bytes(16), true);
    for i in 0..16 {
        assert!(mask.get(Size::from_bytes(i)), "{i} should be set");
    }

    // Grow without requiring an additional block. The gap between the current length and the
    // range's beginning should be set to the same value as the range. Note: since this is fully
    // out-of-bounds of the current mask, this is case #3 described in the `set_range` method.
    let range = (24..32).into();
    mask.set_range(range, false);

    // We want to make sure the unused bits in the first block are correct
    for i in 16..24 {
        assert!(!mask.get(Size::from_bytes(i)), "{i} should not be set");
    }

    for i in 24..32 {
        assert!(!mask.get(Size::from_bytes(i)), "{i} should not be set");
    }

    assert_eq!(1, mask.range_as_init_chunks((0..16).into()).count());
    assert_eq!(2, mask.range_as_init_chunks((0..32).into()).count());
    assert_eq!(materialized_block_count(&mask), 1);
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=grow_within_unused_bits_with_overlap | COMPLEXITY=12 | LINES=27 */

// This is similar to `grow_mixed_state_within_unused_bits` to force materialization, but the range
// to set partially overlaps the mask, so this requires a different growth + write pattern in the
// mask.
#[test]
fn grow_within_unused_bits_with_overlap() {
    // To have spare bits, we use a mask size smaller than its block size of 64.
    let mut mask = InitMask::new(Size::from_bytes(16), true);
    for i in 0..16 {
        assert!(mask.get(Size::from_bytes(i)), "{i} should be set");
    }

    // Grow without requiring an additional block, but leave no gap after the current len. Note:
    // since this is partially out-of-bounds of the current mask, this is case #2 described in the
    // `set_range` method.
    let range = (8..24).into();
    mask.set_range(range, false);

    // We want to make sure the unused bits in the first block are correct
    for i in 8..24 {
        assert!(!mask.get(Size::from_bytes(i)), "{i} should not be set");
    }

    assert_eq!(1, mask.range_as_init_chunks((0..8).into()).count());
    assert_eq!(2, mask.range_as_init_chunks((0..24).into()).count());
    assert_eq!(materialized_block_count(&mask), 1);
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=grow_mixed_state_within_unused_bits_and_full_overwrite | COMPLEXITY=3 | LINES=23 */

// Force materialization before a full overwrite: the mask can now become lazy.
#[test]
fn grow_mixed_state_within_unused_bits_and_full_overwrite() {
    // To have spare bits, we use a mask size smaller than its block size of 64.
    let mut mask = InitMask::new(Size::from_bytes(16), true);
    let range = (0..16).into();
    assert!(mask.is_range_initialized(range).is_ok());

    // Force materialization.
    let range = (8..24).into();
    mask.set_range(range, false);
    assert!(mask.is_range_initialized(range).is_err());
    assert_eq!(materialized_block_count(&mask), 1);

    // Full overwrite, lazy blocks would be enough from now on.
    let range = (0..32).into();
    mask.set_range(range, true);
    assert!(mask.is_range_initialized(range).is_ok());

    assert_eq!(1, mask.range_as_init_chunks((0..32).into()).count());
    assert_eq!(materialized_block_count(&mask), 0);
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=grow_same_state_outside_capacity | COMPLEXITY=9 | LINES=19 */

// Check that growth outside the current capacity can still be lazy: if the init state doesn't
// change, we don't need materialized blocks.
#[test]
fn grow_same_state_outside_capacity() {
    // To have spare bits, we use a mask size smaller than its block size of 64.
    let mut mask = InitMask::new(Size::from_bytes(16), true);
    for i in 0..16 {
        assert!(mask.get(Size::from_bytes(i)), "{i} should be set");
    }
    assert_eq!(materialized_block_count(&mask), 0);

    // Grow to 10 blocks with the same init state.
    let range = (24..640).into();
    mask.set_range(range, true);

    assert_eq!(1, mask.range_as_init_chunks((0..640).into()).count());
    assert_eq!(materialized_block_count(&mask), 0);
}
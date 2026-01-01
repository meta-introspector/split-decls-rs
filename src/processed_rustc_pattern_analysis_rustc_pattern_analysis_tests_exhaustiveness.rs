// SRC: ../rust/compiler/rustc_pattern_analysis/tests/exhaustiveness.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */
// Test exhaustiveness checking.

#[allow(unused_crate_dependencies)]

use common::*;
use crate::rustc_pattern_analysis::MatchArm;
use crate::rustc_pattern_analysis::pat::{DeconstructedPat, WitnessPat};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=run | COMPLEXITY=7 | LINES=23 */
use crate::rustc_pattern_analysis::usefulness::PlaceValidity;

#[macro_use]

/// Analyze a match made of these patterns.
fn run(
    ty: Ty,
    patterns: Vec<DeconstructedPat<Cx>>,
    exhaustive_witnesses: bool,
) -> Vec<WitnessPat<Cx>> {
    let arms: Vec<_> =
        patterns.iter().map(|pat| MatchArm { pat, has_guard: false, arm_data: () }).collect();
    let report = compute_match_usefulness(
        arms.as_slice(),
        ty,
        PlaceValidity::ValidOnly,
        usize::MAX,
        exhaustive_witnesses,
    )
    .unwrap();
    report.non_exhaustiveness_witnesses
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=check | COMPLEXITY=7 | LINES=6 */

/// Analyze a match made of these patterns. Panics if there are no patterns
fn check(patterns: Vec<DeconstructedPat<Cx>>) -> Vec<WitnessPat<Cx>> {
    let ty = *patterns[0].ty();
    run(ty, patterns, true)
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=assert_exhaustive | COMPLEXITY=6 | LINES=8 */

#[track_caller]
fn assert_exhaustive(patterns: Vec<DeconstructedPat<Cx>>) {
    let witnesses = check(patterns);
    if !witnesses.is_empty() {
        panic!("non-exhaustive match: missing {witnesses:?}");
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=assert_non_exhaustive | COMPLEXITY=2 | LINES=6 */

#[track_caller]
fn assert_non_exhaustive(patterns: Vec<DeconstructedPat<Cx>>) {
    let witnesses = check(patterns);
    assert!(!witnesses.is_empty())
}
/* AST_META: AST_ID=6 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

use WhichWitnesses::*;
enum WhichWitnesses {
    AllOfThem,
    OnlySome,
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=assert_witnesses | COMPLEXITY=3 | LINES=14 */

#[track_caller]
/// We take the type as input to support empty matches.
fn assert_witnesses(
    which: WhichWitnesses,
    ty: Ty,
    patterns: Vec<DeconstructedPat<Cx>>,
    expected: Vec<&str>,
) {
    let exhaustive_wit = matches!(which, AllOfThem);
    let witnesses = run(ty, patterns, exhaustive_wit);
    let witnesses: Vec<_> = witnesses.iter().map(|w| format!("{w:?}")).collect();
    assert_eq!(witnesses, expected)
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=test_int_ranges | COMPLEXITY=2 | LINES=22 */

#[test]
fn test_int_ranges() {
    let ty = Ty::U8;
    assert_exhaustive(pats!(ty;
        0..=255,
    ));
    assert_exhaustive(pats!(ty;
        0..,
    ));
    assert_non_exhaustive(pats!(ty;
        0..255,
    ));
    assert_exhaustive(pats!(ty;
        0..255,
        255,
    ));
    assert_exhaustive(pats!(ty;
        ..10,
        10..
    ));
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=test_nested | COMPLEXITY=6 | LINES=23 */

#[test]
fn test_nested() {
    // enum E { A(bool), B(bool) }
    // ty = (E, E)
    let ty = Ty::BigStruct { arity: 2, ty: &Ty::BigEnum { arity: 2, ty: &Ty::Bool } };
    assert_non_exhaustive(pats!(ty;
        Struct(Variant.0, _),
    ));
    assert_exhaustive(pats!(ty;
        Struct(Variant.0, _),
        Struct(Variant.1, _),
    ));
    assert_non_exhaustive(pats!(ty;
        Struct(Variant.0, _),
        Struct(_, Variant.0),
    ));
    assert_exhaustive(pats!(ty;
        Struct(Variant.0, _),
        Struct(_, Variant.0),
        Struct(Variant.1, Variant.1),
    ));
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=test_witnesses | COMPLEXITY=5 | LINES=67 */

#[test]
fn test_witnesses() {
    // TY = Option<bool>
    const TY: Ty = Ty::Enum(&[Ty::Bool, UNIT]);
    // ty = (Option<bool>, Option<bool>)
    let ty = Ty::Tuple(&[TY, TY]);
    assert_witnesses(AllOfThem, ty, vec![], vec!["(_, _)"]);
    assert_witnesses(
        OnlySome,
        ty,
        pats!(ty;
            (Variant.0(false), Variant.0(false)),
        ),
        vec!["(Enum::Variant1(_), _)"],
    );
    assert_witnesses(
        AllOfThem,
        ty,
        pats!(ty;
            (Variant.0(false), Variant.0(false)),
        ),
        vec![
            "(Enum::Variant0(false), Enum::Variant0(true))",
            "(Enum::Variant0(false), Enum::Variant1(_))",
            "(Enum::Variant0(true), _)",
            "(Enum::Variant1(_), _)",
        ],
    );
    assert_witnesses(
        OnlySome,
        ty,
        pats!(ty;
            (_, Variant.0(false)),
        ),
        vec!["(_, Enum::Variant1(_))"],
    );
    assert_witnesses(
        AllOfThem,
        ty,
        pats!(ty;
            (_, Variant.0(false)),
        ),
        vec!["(_, Enum::Variant0(true))", "(_, Enum::Variant1(_))"],
    );

    let ty = Ty::NonExhaustiveEnum(&[UNIT, UNIT, UNIT]);
    assert_witnesses(
        OnlySome,
        ty,
        pats!(ty;
            Variant.0,
        ),
        vec!["_"],
    );
    assert_witnesses(
        AllOfThem,
        ty,
        pats!(ty;
            Variant.0,
        ),
        vec!["Enum::Variant1(_)", "Enum::Variant2(_)", "_"],
    );

    // Assert we put `true` before `false`.
    assert_witnesses(AllOfThem, Ty::Bool, Vec::new(), vec!["true", "false"]);
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=test_empty | COMPLEXITY=2 | LINES=14 */

#[test]
fn test_empty() {
    // `TY = Result<bool, !>`
    const TY: Ty = Ty::Enum(&[Ty::Bool, NEVER]);
    assert_exhaustive(pats!(TY;
        Variant.0,
    ));
    let ty = Ty::Tuple(&[Ty::Bool, TY]);
    assert_exhaustive(pats!(ty;
        (true, Variant.0),
        (false, Variant.0),
    ));
}
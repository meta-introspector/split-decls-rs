
macro_rules! u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u8 in module {}", module_path!());
    };
}

mkfn!{
    u8_introspect!();
    # [inline] pub (super) const fn u8 (val : u8) -> u32 { let val = val as u32 ; const C1 : u32 = 0b11_00000000 - 10 ; const C2 : u32 = 0b10_00000000 - 100 ; ((val + C1) & (val + C2)) >> 8 }
}

macro_rules! less_than_5_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function less_than_5 in module {}", module_path!());
    };
}

mkfn!{
    less_than_5_introspect!();
    # [inline] const fn less_than_5 (val : u32) -> u32 { const C1 : u32 = 0b011_00000000000000000 - 10 ; const C2 : u32 = 0b100_00000000000000000 - 100 ; const C3 : u32 = 0b111_00000000000000000 - 1000 ; const C4 : u32 = 0b100_00000000000000000 - 10000 ; (((val + C1) & (val + C2)) ^ ((val + C3) & (val + C4))) >> 17 }
}

macro_rules! u16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u16 in module {}", module_path!());
    };
}

mkfn!{
    u16_introspect!();
    # [inline] pub (super) const fn u16 (val : u16) -> u32 { less_than_5 (val as u32) }
}

macro_rules! u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u32 in module {}", module_path!());
    };
}

mkfn!{
    u32_introspect!();
    # [inline] pub (super) const fn u32 (mut val : u32) -> u32 { let mut log = 0 ; if val >= 100_000 { val /= 100_000 ; log += 5 ; } log + less_than_5 (val) }
}

macro_rules! u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u64 in module {}", module_path!());
    };
}

mkfn!{
    u64_introspect!();
    # [inline] pub (super) const fn u64 (mut val : u64) -> u32 { let mut log = 0 ; if val >= 10_000_000_000 { val /= 10_000_000_000 ; log += 10 ; } if val >= 100_000 { val /= 100_000 ; log += 5 ; } log + less_than_5 (val as u32) }
}

macro_rules! u128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function u128 in module {}", module_path!());
    };
}

mkfn!{
    u128_introspect!();
    # [inline] pub (super) const fn u128 (mut val : u128) -> u32 { let mut log = 0 ; if val >= 100_000_000_000_000_000_000_000_000_000_000 { val /= 100_000_000_000_000_000_000_000_000_000_000 ; log += 32 ; return log + u32 (val as u32) ; } if val >= 10_000_000_000_000_000 { val /= 10_000_000_000_000_000 ; log += 16 ; } log + u64 (val as u64) }
}

macro_rules! usize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usize in module {}", module_path!());
    };
}

mkfn!{
    usize_introspect!();
    # [cfg (target_pointer_width = "16")] # [inline] pub (super) const fn usize (val : usize) -> u32 { u16 (val as _) }
}

macro_rules! usize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usize in module {}", module_path!());
    };
}

mkfn!{
    usize_introspect!();
    # [cfg (target_pointer_width = "32")] # [inline] pub (super) const fn usize (val : usize) -> u32 { u32 (val as _) }
}

macro_rules! usize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usize in module {}", module_path!());
    };
}

mkfn!{
    usize_introspect!();
    # [cfg (target_pointer_width = "64")] # [inline] pub (super) const fn usize (val : usize) -> u32 { u64 (val as _) }
}

macro_rules! i8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i8 in module {}", module_path!());
    };
}

mkfn!{
    i8_introspect!();
    # [inline] pub (super) const fn i8 (val : i8) -> u32 { u8 (val as u8) }
}

macro_rules! i16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i16 in module {}", module_path!());
    };
}

mkfn!{
    i16_introspect!();
    # [inline] pub (super) const fn i16 (val : i16) -> u32 { u16 (val as u16) }
}

macro_rules! i32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i32 in module {}", module_path!());
    };
}

mkfn!{
    i32_introspect!();
    # [inline] pub (super) const fn i32 (val : i32) -> u32 { u32 (val as u32) }
}

macro_rules! i64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i64 in module {}", module_path!());
    };
}

mkfn!{
    i64_introspect!();
    # [inline] pub (super) const fn i64 (val : i64) -> u32 { u64 (val as u64) }
}

macro_rules! i128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function i128 in module {}", module_path!());
    };
}

mkfn!{
    i128_introspect!();
    # [inline] pub (super) const fn i128 (val : i128) -> u32 { u128 (val as u128) }
}

macro_rules! panic_for_nonpositive_argument_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_for_nonpositive_argument in module {}", module_path!());
    };
}

mkfn!{
    panic_for_nonpositive_argument_introspect!();
    # [doc = " Instantiate this panic logic once, rather than for all the ilog methods"] # [doc = " on every single primitive type."] # [cold] # [track_caller] pub (super) const fn panic_for_nonpositive_argument () -> ! { panic ! ("argument of integer logarithm must be positive") }
}
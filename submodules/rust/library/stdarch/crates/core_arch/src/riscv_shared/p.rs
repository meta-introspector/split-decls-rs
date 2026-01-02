mkuse!{use crate :: arch :: asm ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! add16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add16 in module {}", module_path!());
    };
}

mkfn!{
    add16_introspect!();
    # [doc = " Adds packed 16-bit signed numbers, discarding overflow bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn add16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x20, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! radd16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function radd16 in module {}", module_path!());
    };
}

mkfn!{
    radd16_introspect!();
    # [doc = " Halves the sum of packed 16-bit signed numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn radd16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x00, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! uradd16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function uradd16 in module {}", module_path!());
    };
}

mkfn!{
    uradd16_introspect!();
    # [doc = " Halves the sum of packed 16-bit unsigned numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn uradd16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x10, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kadd16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kadd16 in module {}", module_path!());
    };
}

mkfn!{
    kadd16_introspect!();
    # [doc = " Adds packed 16-bit signed numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kadd16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x08, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ukadd16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ukadd16 in module {}", module_path!());
    };
}

mkfn!{
    ukadd16_introspect!();
    # [doc = " Adds packed 16-bit unsigned numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ukadd16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x18, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! sub16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sub16 in module {}", module_path!());
    };
}

mkfn!{
    sub16_introspect!();
    # [doc = " Subtracts packed 16-bit signed numbers, discarding overflow bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sub16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x21, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! rsub16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rsub16 in module {}", module_path!());
    };
}

mkfn!{
    rsub16_introspect!();
    # [doc = " Halves the subtraction result of packed 16-bit signed numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn rsub16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x01, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ursub16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ursub16 in module {}", module_path!());
    };
}

mkfn!{
    ursub16_introspect!();
    # [doc = " Halves the subtraction result of packed 16-bit unsigned numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ursub16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x11, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ksub16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ksub16 in module {}", module_path!());
    };
}

mkfn!{
    ksub16_introspect!();
    # [doc = " Subtracts packed 16-bit signed numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ksub16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x09, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! uksub16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function uksub16 in module {}", module_path!());
    };
}

mkfn!{
    uksub16_introspect!();
    # [doc = " Subtracts packed 16-bit unsigned numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn uksub16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x19, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! cras16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cras16 in module {}", module_path!());
    };
}

mkfn!{
    cras16_introspect!();
    # [doc = " Cross adds and subtracts packed 16-bit signed numbers, discarding overflow bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn cras16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x22, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! rcras16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rcras16 in module {}", module_path!());
    };
}

mkfn!{
    rcras16_introspect!();
    # [doc = " Cross halves of adds and subtracts packed 16-bit signed numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn rcras16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x02, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! urcras16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function urcras16 in module {}", module_path!());
    };
}

mkfn!{
    urcras16_introspect!();
    # [doc = " Cross halves of adds and subtracts packed 16-bit unsigned numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn urcras16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x12, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kcras16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kcras16 in module {}", module_path!());
    };
}

mkfn!{
    kcras16_introspect!();
    # [doc = " Cross adds and subtracts packed 16-bit signed numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kcras16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x0A, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ukcras16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ukcras16 in module {}", module_path!());
    };
}

mkfn!{
    ukcras16_introspect!();
    # [doc = " Cross adds and subtracts packed 16-bit unsigned numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ukcras16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x1A, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! crsa16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crsa16 in module {}", module_path!());
    };
}

mkfn!{
    crsa16_introspect!();
    # [doc = " Cross subtracts and adds packed 16-bit signed numbers, discarding overflow bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn crsa16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x23, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! rcrsa16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rcrsa16 in module {}", module_path!());
    };
}

mkfn!{
    rcrsa16_introspect!();
    # [doc = " Cross halves of subtracts and adds packed 16-bit signed numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn rcrsa16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x03, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! urcrsa16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function urcrsa16 in module {}", module_path!());
    };
}

mkfn!{
    urcrsa16_introspect!();
    # [doc = " Cross halves of subtracts and adds packed 16-bit unsigned numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn urcrsa16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x13, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kcrsa16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kcrsa16 in module {}", module_path!());
    };
}

mkfn!{
    kcrsa16_introspect!();
    # [doc = " Cross subtracts and adds packed 16-bit signed numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kcrsa16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x0B, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ukcrsa16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ukcrsa16 in module {}", module_path!());
    };
}

mkfn!{
    ukcrsa16_introspect!();
    # [doc = " Cross subtracts and adds packed 16-bit unsigned numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ukcrsa16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x1B, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! stas16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stas16 in module {}", module_path!());
    };
}

mkfn!{
    stas16_introspect!();
    # [doc = " Straight adds and subtracts packed 16-bit signed numbers, discarding overflow bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn stas16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x7A, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! rstas16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rstas16 in module {}", module_path!());
    };
}

mkfn!{
    rstas16_introspect!();
    # [doc = " Straight halves of adds and subtracts packed 16-bit signed numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn rstas16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x5A, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! urstas16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function urstas16 in module {}", module_path!());
    };
}

mkfn!{
    urstas16_introspect!();
    # [doc = " Straight halves of adds and subtracts packed 16-bit unsigned numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn urstas16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x6A, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kstas16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kstas16 in module {}", module_path!());
    };
}

mkfn!{
    kstas16_introspect!();
    # [doc = " Straight adds and subtracts packed 16-bit signed numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kstas16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x62, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ukstas16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ukstas16 in module {}", module_path!());
    };
}

mkfn!{
    ukstas16_introspect!();
    # [doc = " Straight adds and subtracts packed 16-bit unsigned numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ukstas16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x72, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! stsa16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stsa16 in module {}", module_path!());
    };
}

mkfn!{
    stsa16_introspect!();
    # [doc = " Straight subtracts and adds packed 16-bit signed numbers, discarding overflow bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn stsa16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x7B, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! rstsa16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rstsa16 in module {}", module_path!());
    };
}

mkfn!{
    rstsa16_introspect!();
    # [doc = " Straight halves of subtracts and adds packed 16-bit signed numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn rstsa16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x5B, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! urstsa16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function urstsa16 in module {}", module_path!());
    };
}

mkfn!{
    urstsa16_introspect!();
    # [doc = " Straight halves of subtracts and adds packed 16-bit unsigned numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn urstsa16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x6B, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kstsa16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kstsa16 in module {}", module_path!());
    };
}

mkfn!{
    kstsa16_introspect!();
    # [doc = " Straight subtracts and adds packed 16-bit signed numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kstsa16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x63, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ukstsa16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ukstsa16 in module {}", module_path!());
    };
}

mkfn!{
    ukstsa16_introspect!();
    # [doc = " Straight subtracts and adds packed 16-bit unsigned numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ukstsa16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x73, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! add8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add8 in module {}", module_path!());
    };
}

mkfn!{
    add8_introspect!();
    # [doc = " Adds packed 8-bit signed numbers, discarding overflow bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn add8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x24, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! radd8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function radd8 in module {}", module_path!());
    };
}

mkfn!{
    radd8_introspect!();
    # [doc = " Halves the sum of packed 8-bit signed numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn radd8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x04, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! uradd8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function uradd8 in module {}", module_path!());
    };
}

mkfn!{
    uradd8_introspect!();
    # [doc = " Halves the sum of packed 8-bit unsigned numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn uradd8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x14, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kadd8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kadd8 in module {}", module_path!());
    };
}

mkfn!{
    kadd8_introspect!();
    # [doc = " Adds packed 8-bit signed numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kadd8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x0C, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ukadd8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ukadd8 in module {}", module_path!());
    };
}

mkfn!{
    ukadd8_introspect!();
    # [doc = " Adds packed 8-bit unsigned numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ukadd8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x1C, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! sub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sub8 in module {}", module_path!());
    };
}

mkfn!{
    sub8_introspect!();
    # [doc = " Subtracts packed 8-bit signed numbers, discarding overflow bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sub8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x25, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! rsub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rsub8 in module {}", module_path!());
    };
}

mkfn!{
    rsub8_introspect!();
    # [doc = " Halves the subtraction result of packed 8-bit signed numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn rsub8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x05, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ursub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ursub8 in module {}", module_path!());
    };
}

mkfn!{
    ursub8_introspect!();
    # [doc = " Halves the subtraction result of packed 8-bit unsigned numbers, dropping least bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ursub8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x15, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ksub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ksub8 in module {}", module_path!());
    };
}

mkfn!{
    ksub8_introspect!();
    # [doc = " Subtracts packed 8-bit signed numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ksub8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x0D, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! uksub8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function uksub8 in module {}", module_path!());
    };
}

mkfn!{
    uksub8_introspect!();
    # [doc = " Subtracts packed 8-bit unsigned numbers, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn uksub8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x1D, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! sra16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sra16 in module {}", module_path!());
    };
}

mkfn!{
    sra16_introspect!();
    # [doc = " Arithmetic right shift packed 16-bit elements without rounding up"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sra16 (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x28, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! sra16u_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sra16u in module {}", module_path!());
    };
}

mkfn!{
    sra16u_introspect!();
    # [doc = " Arithmetic right shift packed 16-bit elements with rounding up"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sra16u (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x30, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! srl16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function srl16 in module {}", module_path!());
    };
}

mkfn!{
    srl16_introspect!();
    # [doc = " Logical right shift packed 16-bit elements without rounding up"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn srl16 (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x29, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! srl16u_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function srl16u in module {}", module_path!());
    };
}

mkfn!{
    srl16u_introspect!();
    # [doc = " Logical right shift packed 16-bit elements with rounding up"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn srl16u (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x31, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! sll16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sll16 in module {}", module_path!());
    };
}

mkfn!{
    sll16_introspect!();
    # [doc = " Logical left shift packed 16-bit elements, discarding overflow bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sll16 (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x2A, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ksll16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ksll16 in module {}", module_path!());
    };
}

mkfn!{
    ksll16_introspect!();
    # [doc = " Logical left shift packed 16-bit elements, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ksll16 (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x32, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kslra16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kslra16 in module {}", module_path!());
    };
}

mkfn!{
    kslra16_introspect!();
    # [doc = " Logical saturating left then arithmetic right shift packed 16-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kslra16 (a : usize , b : i32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x2B, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kslra16u_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kslra16u in module {}", module_path!());
    };
}

mkfn!{
    kslra16u_introspect!();
    # [doc = " Logical saturating left then arithmetic right shift packed 16-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kslra16u (a : usize , b : i32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x33, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! sra8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sra8 in module {}", module_path!());
    };
}

mkfn!{
    sra8_introspect!();
    # [doc = " Arithmetic right shift packed 8-bit elements without rounding up"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sra8 (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x2C, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! sra8u_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sra8u in module {}", module_path!());
    };
}

mkfn!{
    sra8u_introspect!();
    # [doc = " Arithmetic right shift packed 8-bit elements with rounding up"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sra8u (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x34, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! srl8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function srl8 in module {}", module_path!());
    };
}

mkfn!{
    srl8_introspect!();
    # [doc = " Logical right shift packed 8-bit elements without rounding up"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn srl8 (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x2D, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! srl8u_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function srl8u in module {}", module_path!());
    };
}

mkfn!{
    srl8u_introspect!();
    # [doc = " Logical right shift packed 8-bit elements with rounding up"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn srl8u (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x35, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! sll8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sll8 in module {}", module_path!());
    };
}

mkfn!{
    sll8_introspect!();
    # [doc = " Logical left shift packed 8-bit elements, discarding overflow bits"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sll8 (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x2E, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ksll8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ksll8 in module {}", module_path!());
    };
}

mkfn!{
    ksll8_introspect!();
    # [doc = " Logical left shift packed 8-bit elements, saturating at the numeric bounds"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ksll8 (a : usize , b : u32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x36, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kslra8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kslra8 in module {}", module_path!());
    };
}

mkfn!{
    kslra8_introspect!();
    # [doc = " Logical saturating left then arithmetic right shift packed 8-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kslra8 (a : usize , b : i32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x2F, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kslra8u_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kslra8u in module {}", module_path!());
    };
}

mkfn!{
    kslra8u_introspect!();
    # [doc = " Logical saturating left then arithmetic right shift packed 8-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kslra8u (a : usize , b : i32) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x37, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! cmpeq16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cmpeq16 in module {}", module_path!());
    };
}

mkfn!{
    cmpeq16_introspect!();
    # [doc = " Compare equality for packed 16-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn cmpeq16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x26, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! scmplt16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scmplt16 in module {}", module_path!());
    };
}

mkfn!{
    scmplt16_introspect!();
    # [doc = " Compare whether 16-bit packed signed integers are less than the others"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn scmplt16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x06, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! scmple16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scmple16 in module {}", module_path!());
    };
}

mkfn!{
    scmple16_introspect!();
    # [doc = " Compare whether 16-bit packed signed integers are less than or equal to the others"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn scmple16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x0E, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ucmplt16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ucmplt16 in module {}", module_path!());
    };
}

mkfn!{
    ucmplt16_introspect!();
    # [doc = " Compare whether 16-bit packed unsigned integers are less than the others"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ucmplt16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x16, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ucmple16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ucmple16 in module {}", module_path!());
    };
}

mkfn!{
    ucmple16_introspect!();
    # [doc = " Compare whether 16-bit packed unsigned integers are less than or equal to the others"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ucmple16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x1E, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! cmpeq8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cmpeq8 in module {}", module_path!());
    };
}

mkfn!{
    cmpeq8_introspect!();
    # [doc = " Compare equality for packed 8-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn cmpeq8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x27, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! scmplt8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scmplt8 in module {}", module_path!());
    };
}

mkfn!{
    scmplt8_introspect!();
    # [doc = " Compare whether 8-bit packed signed integers are less than the others"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn scmplt8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x07, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! scmple8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scmple8 in module {}", module_path!());
    };
}

mkfn!{
    scmple8_introspect!();
    # [doc = " Compare whether 8-bit packed signed integers are less than or equal to the others"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn scmple8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x0F, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ucmplt8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ucmplt8 in module {}", module_path!());
    };
}

mkfn!{
    ucmplt8_introspect!();
    # [doc = " Compare whether 8-bit packed unsigned integers are less than the others"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ucmplt8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x17, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ucmple8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ucmple8 in module {}", module_path!());
    };
}

mkfn!{
    ucmple8_introspect!();
    # [doc = " Compare whether 8-bit packed unsigned integers are less than or equal to the others"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ucmple8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x1F, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! smin16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smin16 in module {}", module_path!());
    };
}

mkfn!{
    smin16_introspect!();
    # [doc = " Get minimum values from 16-bit packed signed integers"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn smin16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x40, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! umin16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function umin16 in module {}", module_path!());
    };
}

mkfn!{
    umin16_introspect!();
    # [doc = " Get minimum values from 16-bit packed unsigned integers"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn umin16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x48, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! smax16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smax16 in module {}", module_path!());
    };
}

mkfn!{
    smax16_introspect!();
    # [doc = " Get maximum values from 16-bit packed signed integers"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn smax16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x41, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! umax16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function umax16 in module {}", module_path!());
    };
}

mkfn!{
    umax16_introspect!();
    # [doc = " Get maximum values from 16-bit packed unsigned integers"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn umax16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x49, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kabs16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kabs16 in module {}", module_path!());
    };
}

mkfn!{
    kabs16_introspect!();
    # [doc = " Compute the absolute value of packed 16-bit signed integers"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kabs16 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAD1)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! clrs16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clrs16 in module {}", module_path!());
    };
}

mkfn!{
    clrs16_introspect!();
    # [doc = " Count the number of redundant sign bits of the packed 16-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn clrs16 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAE8)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! clz16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clz16 in module {}", module_path!());
    };
}

mkfn!{
    clz16_introspect!();
    # [doc = " Count the number of leading zero bits of the packed 16-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn clz16 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAE9)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! swap16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function swap16 in module {}", module_path!());
    };
}

mkfn!{
    swap16_introspect!();
    # [doc = " Swap the 16-bit halfwords within each 32-bit word of a register"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn swap16 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x0F, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! smin8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smin8 in module {}", module_path!());
    };
}

mkfn!{
    smin8_introspect!();
    # [doc = " Get minimum values from 8-bit packed signed integers"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn smin8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x44, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! umin8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function umin8 in module {}", module_path!());
    };
}

mkfn!{
    umin8_introspect!();
    # [doc = " Get minimum values from 8-bit packed unsigned integers"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn umin8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x4C, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! smax8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smax8 in module {}", module_path!());
    };
}

mkfn!{
    smax8_introspect!();
    # [doc = " Get maximum values from 8-bit packed signed integers"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn smax8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x45, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! umax8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function umax8 in module {}", module_path!());
    };
}

mkfn!{
    umax8_introspect!();
    # [doc = " Get maximum values from 8-bit packed unsigned integers"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn umax8 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x4D, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kabs8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kabs8 in module {}", module_path!());
    };
}

mkfn!{
    kabs8_introspect!();
    # [doc = " Compute the absolute value of packed 8-bit signed integers"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kabs8 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAD0)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! clrs8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clrs8 in module {}", module_path!());
    };
}

mkfn!{
    clrs8_introspect!();
    # [doc = " Count the number of redundant sign bits of the packed 8-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn clrs8 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAE0)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! clz8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clz8 in module {}", module_path!());
    };
}

mkfn!{
    clz8_introspect!();
    # [doc = " Count the number of leading zero bits of the packed 8-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn clz8 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAE1)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! swap8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function swap8 in module {}", module_path!());
    };
}

mkfn!{
    swap8_introspect!();
    # [doc = " Swap the 8-bit bytes within each 16-bit halfword of a register."] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn swap8 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAD8)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! sunpkd810_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sunpkd810 in module {}", module_path!());
    };
}

mkfn!{
    sunpkd810_introspect!();
    # [doc = " Unpack first and zeroth into two 16-bit signed halfwords in each 32-bit chunk"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sunpkd810 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAC8)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! sunpkd820_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sunpkd820 in module {}", module_path!());
    };
}

mkfn!{
    sunpkd820_introspect!();
    # [doc = " Unpack second and zeroth into two 16-bit signed halfwords in each 32-bit chunk"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sunpkd820 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAC9)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! sunpkd830_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sunpkd830 in module {}", module_path!());
    };
}

mkfn!{
    sunpkd830_introspect!();
    # [doc = " Unpack third and zeroth into two 16-bit signed halfwords in each 32-bit chunk"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sunpkd830 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xACA)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! sunpkd831_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sunpkd831 in module {}", module_path!());
    };
}

mkfn!{
    sunpkd831_introspect!();
    # [doc = " Unpack third and first into two 16-bit signed halfwords in each 32-bit chunk"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sunpkd831 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xACB)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! sunpkd832_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sunpkd832 in module {}", module_path!());
    };
}

mkfn!{
    sunpkd832_introspect!();
    # [doc = " Unpack third and second into two 16-bit signed halfwords in each 32-bit chunk"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn sunpkd832 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAD3)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! zunpkd810_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zunpkd810 in module {}", module_path!());
    };
}

mkfn!{
    zunpkd810_introspect!();
    # [doc = " Unpack first and zeroth into two 16-bit unsigned halfwords in each 32-bit chunk"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn zunpkd810 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xACC)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! zunpkd820_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zunpkd820 in module {}", module_path!());
    };
}

mkfn!{
    zunpkd820_introspect!();
    # [doc = " Unpack second and zeroth into two 16-bit unsigned halfwords in each 32-bit chunk"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn zunpkd820 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xACD)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! zunpkd830_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zunpkd830 in module {}", module_path!());
    };
}

mkfn!{
    zunpkd830_introspect!();
    # [doc = " Unpack third and zeroth into two 16-bit unsigned halfwords in each 32-bit chunk"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn zunpkd830 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xACE)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! zunpkd831_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zunpkd831 in module {}", module_path!());
    };
}

mkfn!{
    zunpkd831_introspect!();
    # [doc = " Unpack third and first into two 16-bit unsigned halfwords in each 32-bit chunk"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn zunpkd831 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xACF)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! zunpkd832_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zunpkd832 in module {}", module_path!());
    };
}

mkfn!{
    zunpkd832_introspect!();
    # [doc = " Unpack third and second into two 16-bit unsigned halfwords in each 32-bit chunk"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn zunpkd832 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAD7)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! pkbt16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pkbt16 in module {}", module_path!());
    };
}

mkfn!{
    pkbt16_introspect!();
    # [doc = " Pack two 16-bit data from bottom and top half from 32-bit chunks"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn pkbt16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x1, 0x0F, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! pktb16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pktb16 in module {}", module_path!());
    };
}

mkfn!{
    pktb16_introspect!();
    # [doc = " Pack two 16-bit data from top and bottom half from 32-bit chunks"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn pktb16 (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x1, 0x1F, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! clrs32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clrs32 in module {}", module_path!());
    };
}

mkfn!{
    clrs32_introspect!();
    # [doc = " Count the number of redundant sign bits of the packed 32-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn clrs32 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAF8)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! clz32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clz32 in module {}", module_path!());
    };
}

mkfn!{
    clz32_introspect!();
    # [doc = " Count the number of leading zero bits of the packed 32-bit elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn clz32 (a : usize) -> usize { let value : usize ; unsafe { asm ! (".insn i 0x77, 0x0, {}, {}, %lo(0xAF9)" , lateout (reg) value , in (reg) a , options (pure , nomem , nostack)) } value }
}

macro_rules! pbsad_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pbsad in module {}", module_path!());
    };
}

mkfn!{
    pbsad_introspect!();
    # [doc = " Calculate the sum of absolute difference of unsigned 8-bit data elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn pbsad (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x7E, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! pbsada_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pbsada in module {}", module_path!());
    };
}

mkfn!{
    pbsada_introspect!();
    # [doc = " Calculate and accumulate the sum of absolute difference of unsigned 8-bit data elements"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn pbsada (t : usize , a : usize , b : usize) -> usize { let mut value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x7F, {}, {}, {}" , inlateout (reg) t => value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! smaqa_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smaqa in module {}", module_path!());
    };
}

mkfn!{
    smaqa_introspect!();
    # [doc = " Multiply signed 8-bit elements and add 16-bit elements on results for packed 32-bit chunks"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn smaqa (t : usize , a : usize , b : usize) -> usize { let mut value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x64, {}, {}, {}" , inlateout (reg) t => value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! umaqa_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function umaqa in module {}", module_path!());
    };
}

mkfn!{
    umaqa_introspect!();
    # [doc = " Multiply unsigned 8-bit elements and add 16-bit elements on results for packed 32-bit chunks"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn umaqa (t : usize , a : usize , b : usize) -> usize { let mut value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x66, {}, {}, {}" , inlateout (reg) t => value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! smaqasu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smaqasu in module {}", module_path!());
    };
}

mkfn!{
    smaqasu_introspect!();
    # [doc = " Multiply signed to unsigned 8-bit and add 16-bit elements on results for packed 32-bit chunks"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn smaqasu (t : usize , a : usize , b : usize) -> usize { let mut value : usize ; unsafe { asm ! (".insn r 0x77, 0x0, 0x65, {}, {}, {}" , inlateout (reg) t => value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! kaddh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kaddh in module {}", module_path!());
    };
}

mkfn!{
    kaddh_introspect!();
    # [doc = " Adds signed lower 16-bit content of two registers with Q15 saturation"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn kaddh (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x1, 0x02, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ksubh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ksubh in module {}", module_path!());
    };
}

mkfn!{
    ksubh_introspect!();
    # [doc = " Subtracts signed lower 16-bit content of two registers with Q15 saturation"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ksubh (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x1, 0x03, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! ukaddh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ukaddh in module {}", module_path!());
    };
}

mkfn!{
    ukaddh_introspect!();
    # [doc = " Adds signed lower 16-bit content of two registers with U16 saturation"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn ukaddh (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x1, 0x0A, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}

macro_rules! uksubh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function uksubh in module {}", module_path!());
    };
}

mkfn!{
    uksubh_introspect!();
    # [doc = " Subtracts signed lower 16-bit content of two registers with U16 saturation"] # [inline] # [cfg_attr (test , assert_instr (unknown))] # [unstable (feature = "riscv_ext_intrinsics" , issue = "114544")] pub fn uksubh (a : usize , b : usize) -> usize { let value : usize ; unsafe { asm ! (".insn r 0x77, 0x1, 0x0B, {}, {}, {}" , lateout (reg) value , in (reg) a , in (reg) b , options (pure , nomem , nostack)) } value }
}
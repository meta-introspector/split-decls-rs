mkuse!{use core :: arch :: asm ;}
mkuse!{use crate :: detect :: { Feature , cache } ;}

macro_rules! detect_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_features in module {}", module_path!());
    };
}

mkfn!{
    detect_features_introspect!();
    # [doc = " Try to read the features from the system registers."] # [doc = ""] # [doc = " This will cause SIGILL if the current OS is not trapping the mrs instruction."] pub (crate) fn detect_features () -> cache :: Initializer { let aa64isar0 : u64 ; unsafe { asm ! ("mrs {}, ID_AA64ISAR0_EL1" , out (reg) aa64isar0 , options (pure , nomem , preserves_flags , nostack)) ; } let aa64isar1 : u64 ; unsafe { asm ! ("mrs {}, ID_AA64ISAR1_EL1" , out (reg) aa64isar1 , options (pure , nomem , preserves_flags , nostack)) ; } let aa64mmfr2 : u64 ; unsafe { asm ! ("mrs {}, ID_AA64MMFR2_EL1" , out (reg) aa64mmfr2 , options (pure , nomem , preserves_flags , nostack)) ; } let aa64pfr0 : u64 ; unsafe { asm ! ("mrs {}, ID_AA64PFR0_EL1" , out (reg) aa64pfr0 , options (pure , nomem , preserves_flags , nostack)) ; } parse_system_registers (aa64isar0 , aa64isar1 , aa64mmfr2 , Some (aa64pfr0)) }
}

macro_rules! parse_system_registers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_system_registers in module {}", module_path!());
    };
}

mkfn!{
    parse_system_registers_introspect!();
    pub (crate) fn parse_system_registers (aa64isar0 : u64 , aa64isar1 : u64 , aa64mmfr2 : u64 , aa64pfr0 : Option < u64 > ,) -> cache :: Initializer { let mut value = cache :: Initializer :: default () ; let mut enable_feature = | f , enable | { if enable { value . set (f as u32) ; } } ; enable_feature (Feature :: pmull , bits_shift (aa64isar0 , 7 , 4) >= 2) ; enable_feature (Feature :: tme , bits_shift (aa64isar0 , 27 , 24) == 1) ; enable_feature (Feature :: lse , bits_shift (aa64isar0 , 23 , 20) >= 2) ; enable_feature (Feature :: crc , bits_shift (aa64isar0 , 19 , 16) >= 1) ; if let Some (aa64pfr0) = aa64pfr0 { let fp = bits_shift (aa64pfr0 , 19 , 16) < 0xF ; let fphp = bits_shift (aa64pfr0 , 19 , 16) >= 1 ; let asimd = bits_shift (aa64pfr0 , 23 , 20) < 0xF ; let asimdhp = bits_shift (aa64pfr0 , 23 , 20) >= 1 ; enable_feature (Feature :: fp , fp) ; enable_feature (Feature :: fp16 , fphp) ; enable_feature (Feature :: asimd , fp && asimd && (! fphp | asimdhp)) ; enable_feature (Feature :: aes , asimd && bits_shift (aa64isar0 , 7 , 4) >= 2) ; let sha1 = bits_shift (aa64isar0 , 11 , 8) >= 1 ; let sha2 = bits_shift (aa64isar0 , 15 , 12) >= 1 ; enable_feature (Feature :: sha2 , asimd && sha1 && sha2) ; enable_feature (Feature :: rdm , asimd && bits_shift (aa64isar0 , 31 , 28) >= 1) ; enable_feature (Feature :: dotprod , asimd && bits_shift (aa64isar0 , 47 , 44) >= 1) ; enable_feature (Feature :: sve , asimd && bits_shift (aa64pfr0 , 35 , 32) >= 1) ; } enable_feature (Feature :: paca , bits_shift (aa64isar1 , 11 , 4) >= 1) ; enable_feature (Feature :: rcpc , bits_shift (aa64isar1 , 23 , 20) >= 1) ; enable_feature (Feature :: pacg , bits_shift (aa64isar1 , 31 , 24) >= 1) ; enable_feature (Feature :: lse2 , bits_shift (aa64mmfr2 , 35 , 32) >= 1) ; value }
}

macro_rules! bits_shift_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bits_shift in module {}", module_path!());
    };
}

mkfn!{
    bits_shift_introspect!();
    # [inline] fn bits_shift (x : u64 , high : usize , low : usize) -> u64 { (x >> low) & ((1 << (high - low + 1)) - 1) }
}
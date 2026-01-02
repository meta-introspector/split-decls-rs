mkuse!{use super :: auxvec ;}
mkuse!{use crate :: detect :: { Feature , cache } ;}
mkitem!{const HWCAP_NEON : usize = 0x00001000 ;}
mkitem!{const HWCAP2_AES : usize = 0x00000001 ;}
mkitem!{const HWCAP2_PMULL : usize = 0x00000002 ;}
mkitem!{const HWCAP2_SHA1 : usize = 0x00000004 ;}
mkitem!{const HWCAP2_SHA2 : usize = 0x00000008 ;}
mkitem!{const HWCAP2_CRC32 : usize = 0x00000010 ;}

macro_rules! detect_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_features in module {}", module_path!());
    };
}

mkfn!{
    detect_features_introspect!();
    # [doc = " Try to read the features from the auxiliary vector"] pub (crate) fn detect_features () -> cache :: Initializer { let mut value = cache :: Initializer :: default () ; let enable_feature = | value : & mut cache :: Initializer , f , enable | { if enable { value . set (f as u32) ; } } ; if let Ok (auxv) = auxvec :: auxv () { enable_feature (& mut value , Feature :: neon , auxv . hwcap & HWCAP_NEON != 0) ; enable_feature (& mut value , Feature :: pmull , auxv . hwcap2 & HWCAP2_PMULL != 0) ; enable_feature (& mut value , Feature :: crc , auxv . hwcap2 & HWCAP2_CRC32 != 0) ; enable_feature (& mut value , Feature :: aes , auxv . hwcap2 & HWCAP2_AES != 0) ; let sha1 = auxv . hwcap2 & HWCAP2_SHA1 != 0 ; let sha2 = auxv . hwcap2 & HWCAP2_SHA2 != 0 ; enable_feature (& mut value , Feature :: sha2 , sha1 && sha2) ; return value ; } value }
}
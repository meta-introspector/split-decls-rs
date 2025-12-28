macro_rules! deps {
    () => {
        FilterConfig!();
        FilterType!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl FilterConfig { # [doc = " Creates a new delta filter configuration."] pub fn new_delta (distance : u32) -> Self { Self { filter_type : FilterType :: Delta , property : distance , } } # [doc = " Creates a new BCJ x86 filter configuration."] pub fn new_bcj_x86 (start_pos : u32) -> Self { Self { filter_type : FilterType :: BcjX86 , property : start_pos , } } # [doc = " Creates a new BCJ ARM filter configuration."] pub fn new_bcj_arm (start_pos : u32) -> Self { Self { filter_type : FilterType :: BcjArm , property : start_pos , } } # [doc = " Creates a new BCJ ARM Thumb filter configuration."] pub fn new_bcj_arm_thumb (start_pos : u32) -> Self { Self { filter_type : FilterType :: BcjArmThumb , property : start_pos , } } # [doc = " Creates a new BCJ ARM64 filter configuration."] pub fn new_bcj_arm64 (start_pos : u32) -> Self { Self { filter_type : FilterType :: BcjArm64 , property : start_pos , } } # [doc = " Creates a new BCJ IA64 filter configuration."] pub fn new_bcj_ia64 (start_pos : u32) -> Self { Self { filter_type : FilterType :: BcjIa64 , property : start_pos , } } # [doc = " Creates a new BCJ PPC filter configuration."] pub fn new_bcj_ppc (start_pos : u32) -> Self { Self { filter_type : FilterType :: BcjPpc , property : start_pos , } } # [doc = " Creates a new BCJ SPARC filter configuration."] pub fn new_bcj_sparc (start_pos : u32) -> Self { Self { filter_type : FilterType :: BcjSparc , property : start_pos , } } # [doc = " Creates a new BCJ RISC-V filter configuration."] pub fn new_bcj_risc_v (start_pos : u32) -> Self { Self { filter_type : FilterType :: BcjRiscv , property : start_pos , } } }
    };
}

impl_145!();
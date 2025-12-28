macro_rules! IMP_EMU_SUB_WORD_CAS {
    () => {
        # [cfg (valgrind)] pub (crate) const IMP_EMU_SUB_WORD_CAS : bool = cfg ! (target_arch = "s390x") || IMP_ARM_LINUX ;
    };
}

IMP_EMU_SUB_WORD_CAS!()
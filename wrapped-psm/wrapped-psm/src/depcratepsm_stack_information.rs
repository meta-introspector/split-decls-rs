// Generated macro for psm_stack_information (macro)
macro_rules! Depcratepsm_stack_information {
() => {
// Module: crate
// Provides: {"psm_stack_information"}
// Dependencies: {}
# [doc = " Macro that outputs its tokens only if `psm::stack_pointer` and `psm::StackDirection::new` are"] # [doc = " available."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use psm::psm_stack_information;"] # [doc = " psm_stack_information! {"] # [doc = "     yes {"] # [doc = "         /* `psm::stack_pointer` and `psm::StackDirection::new` are available here */"] # [doc = "     }"] # [doc = "     no {"] # [doc = "         /* `psm::stack_pointer` and `psm::StackDirection::new` are not available here */"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [cfg (not (asm))] # [macro_export] macro_rules ! psm_stack_information { (yes { $ ($ yes : tt) * } no { $ ($ no : tt) * }) => { $ ($ no) * } ; }
};
}

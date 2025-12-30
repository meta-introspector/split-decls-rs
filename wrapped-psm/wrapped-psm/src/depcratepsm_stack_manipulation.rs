// Generated macro for psm_stack_manipulation (macro)
macro_rules! Depcratepsm_stack_manipulation {
() => {
// Module: crate
// Provides: {"psm_stack_manipulation"}
// Dependencies: {}
# [doc = " Macro that outputs its tokens only if `psm::on_stack` and `psm::replace_stack` are available."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use psm::psm_stack_manipulation;"] # [doc = " psm_stack_manipulation! {"] # [doc = "     yes {"] # [doc = "         /* Functions `on_stack` and `replace_stack` are available here */"] # [doc = "     }"] # [doc = "     no {"] # [doc = "         /* Functions `on_stack` and `replace_stack` are not available here */"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [cfg (not (switchable_stack))] # [macro_export] macro_rules ! psm_stack_manipulation { (yes { $ ($ yes : tt) * } no { $ ($ no : tt) * }) => { $ ($ no) * } ; }
};
}

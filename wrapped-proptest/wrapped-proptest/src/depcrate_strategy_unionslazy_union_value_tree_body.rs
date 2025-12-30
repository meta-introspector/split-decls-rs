// Generated macro for lazy_union_value_tree_body (macro)
macro_rules! Depcrate_strategy_unionslazy_union_value_tree_body {
() => {
// Module: crate::strategy::unions
// Provides: {"lazy_union_value_tree_body"}
// Dependencies: {}
macro_rules ! lazy_union_value_tree_body { ($ typ : ty , $ access : ident) => { type Value = $ typ ; fn current (& self) -> Self :: Value { $ access ! ([] opt = self , self . pick , { opt . as_inner () . unwrap_or_else (|| panic ! ("value tree at self.pick = {} must be initialized" , self . pick ,)) . current () }) } fn simplify (& mut self) -> bool { let orig_pick = self . pick ; if $ access ! ([mut] opt = self , orig_pick , { opt . as_inner_mut () . unwrap_or_else (|| panic ! ("value tree at self.pick = {} must be initialized" , orig_pick ,)) . simplify () }) { self . prev_pick = None ; return true ; } assert ! (self . pick >= self . min_pick , "self.pick = {} should never go below self.min_pick = {}" , self . pick , self . min_pick ,) ; if self . pick == self . min_pick { return false ; } self . prev_pick = Some (self . pick) ; let mut next_pick = self . pick ; while next_pick > self . min_pick { next_pick -= 1 ; let initialized = $ access ! ([mut] opt = self , next_pick , { opt . maybe_init () ; opt . is_initialized () }) ; if initialized { self . pick = next_pick ; return true ; } } false } fn complicate (& mut self) -> bool { if let Some (pick) = self . prev_pick { self . pick = pick ; self . min_pick = pick ; self . prev_pick = None ; true } else { let pick = self . pick ; $ access ! ([mut] opt = self , pick , { opt . as_inner_mut () . unwrap_or_else (|| panic ! ("value tree at self.pick = {} must be initialized" , pick ,)) . complicate () }) } } } }
};
}

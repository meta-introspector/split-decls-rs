// Generated macro for require (macro)
macro_rules! Depcrate_utilsrequire {
() => {
// Module: crate::utils
// Provides: {"require"}
// Dependencies: {}
macro_rules ! require { ($ test : expr ; $ ($ cond : expr ;) *) => { let test = $ test ; let mut cond = true ; if let Some (target) = test . target . as_ref () { cond &= function_name ! () . contains (target) ; } $ (cond &= $ cond ;) * if ! cond { return Ok (()) ; } test . count . set (test . count . get () + 1) ; } }
};
}

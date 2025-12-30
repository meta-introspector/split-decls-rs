// Generated macro for Constant (struct)
macro_rules! Depcrate_accessConstant {
() => {
// Module: crate::access
// Provides: {"Constant"}
// Dependencies: {}
# [doc = " Access to an constant."] # [doc = ""] # [doc = " This wraps a constant value to provide [`Access`] to it. It is constant in the sense that,"] # [doc = " unlike [`ArcSwapAny`] and [`Map`], the loaded value will always stay the same (there's no"] # [doc = " remote `store`)."] # [doc = ""] # [doc = " The purpose is mostly testing and plugging a parameter that works generically from code that"] # [doc = " doesn't need the updating functionality."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct Constant < T > (pub T) ;
};
}

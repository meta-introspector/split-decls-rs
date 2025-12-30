// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < T , E > COption < Result < T , E > > { # [doc = " Transposes an `COption` of a [`Result`] into a [`Result`] of an `COption`."] # [doc = ""] # [doc = " [`COption::None`] will be mapped to [`Ok`]`(`[`COption::None`]`)`."] # [doc = " [`COption::Some`]`(`[`Ok`]`(_))` and [`COption::Some`]`(`[`Err`]`(_))` will be mapped to"] # [doc = " [`Ok`]`(`[`COption::Some`]`(_))` and [`Err`]`(_)`."] # [doc = ""] # [doc = " [`COption::None`]: #variant.COption::None"] # [doc = " [`Ok`]: ../../std/result/enum.Result.html#variant.Ok"] # [doc = " [`COption::Some`]: #variant.COption::Some"] # [doc = " [`Err`]: ../../std/result/enum.Result.html#variant.Err"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[derive(Debug, Eq, PartialEq)]"] # [doc = " struct COption::SomeErr;"] # [doc = ""] # [doc = " let x: Result<COption<i32>, COption::SomeErr> = Ok(COption::Some(5));"] # [doc = " let y: COption<Result<i32, COption::SomeErr>> = COption::Some(Ok(5));"] # [doc = " assert_eq!(x, y.transpose());"] # [doc = " ```"] # [inline] pub fn transpose (self) -> Result < COption < T > , E > { match self { COption :: Some (Ok (x)) => Ok (COption :: Some (x)) , COption :: Some (Err (e)) => Err (e) , COption :: None => Ok (COption :: None) , } } }
};
}

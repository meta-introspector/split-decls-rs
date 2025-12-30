// Generated macro for impl_347 (impl)
macro_rules! Depcrate_pertimpl_347 {
() => {
// Module: crate::pert
// Provides: {"impl_347"}
// Dependencies: {}
impl < F > PertBuilder < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [doc = " Set the shape parameter"] # [doc = ""] # [doc = " If not specified, this defaults to 4."] # [inline] pub fn with_shape (mut self , shape : F) -> PertBuilder < F > { self . shape = shape ; self } # [doc = " Specify the mean"] # [inline] pub fn with_mean (self , mean : F) -> Result < Pert < F > , PertError > { let two = F :: from (2.0) . unwrap () ; let mode = ((self . shape + two) * mean - self . min - self . max) / self . shape ; self . with_mode (mode) } # [doc = " Specify the mode"] # [inline] pub fn with_mode (self , mode : F) -> Result < Pert < F > , PertError > { if ! (self . max > self . min) { return Err (PertError :: RangeTooSmall) ; } if ! (mode >= self . min && self . max >= mode) { return Err (PertError :: ModeRange) ; } if ! (self . shape >= F :: from (0.) . unwrap ()) { return Err (PertError :: ShapeTooSmall) ; } let (min , max , shape) = (self . min , self . max , self . shape) ; let range = max - min ; let v = F :: from (1.0) . unwrap () + shape * (mode - min) / range ; let w = F :: from (1.0) . unwrap () + shape * (max - mode) / range ; let beta = Beta :: new (v , w) . map_err (| _ | PertError :: RangeTooSmall) ? ; Ok (Pert { min , range , beta }) } }
};
}

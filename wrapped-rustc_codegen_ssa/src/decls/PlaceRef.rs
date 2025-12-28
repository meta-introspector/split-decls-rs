macro_rules! deps {
    () => {
        PlaceValue!();
    };
}

macro_rules! PlaceRef {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] pub struct PlaceRef < 'tcx , V > { # [doc = " The location and extra runtime properties of the place."] pub val : PlaceValue < V > , # [doc = " The monomorphized type of this place, including variant information."] # [doc = ""] # [doc = " You probably shouldn't use the alignment from this layout;"] # [doc = " rather you should use the `.val.align` of the actual place,"] # [doc = " which might be different from the type's normal alignment."] pub layout : TyAndLayout < 'tcx > , }
    };
}

PlaceRef!();
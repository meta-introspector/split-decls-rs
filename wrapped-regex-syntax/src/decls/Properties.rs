macro_rules! deps {
    () => {
        PropertiesI!();
    };
}

macro_rules! Properties {
    () => {
        deps!();
        # [doc = " A type that collects various properties of an HIR value."] # [doc = ""] # [doc = " Properties are always scalar values and represent meta data that is"] # [doc = " computed inductively on an HIR value. Properties are defined for all"] # [doc = " HIR values."] # [doc = ""] # [doc = " All methods on a `Properties` value take constant time and are meant to"] # [doc = " be cheap to call."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Properties (Box < PropertiesI >) ;
    };
}

Properties!();
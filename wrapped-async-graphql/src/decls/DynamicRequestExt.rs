macro_rules! deps {
    () => {
        DynamicRequest!();
        FieldValue!();
    };
}

macro_rules! DynamicRequestExt {
    () => {
        deps!();
        # [doc = " A trait for [`DynamicRequest`]"] pub trait DynamicRequestExt { # [doc = " Specify the root value for the request"] fn root_value (self , value : FieldValue < 'static >) -> DynamicRequest ; }
    };
}

DynamicRequestExt!()
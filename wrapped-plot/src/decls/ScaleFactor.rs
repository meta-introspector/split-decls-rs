macro_rules! deps {
    () => {
        Axis!();
    };
}

macro_rules! ScaleFactor {
    () => {
        deps!();
        # [doc = " Axis scale factor"] # [allow (missing_docs)] # [derive (Clone , Copy)] pub struct ScaleFactor (pub f64) ;
    };
}

ScaleFactor!();
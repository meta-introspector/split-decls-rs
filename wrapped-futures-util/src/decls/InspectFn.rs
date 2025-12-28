macro_rules! InspectFn {
    () => {
        # [derive (Debug , Copy , Clone , Default)] pub struct InspectFn < F > (F) ;
    };
}

InspectFn!();
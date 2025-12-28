macro_rules! ComdatId {
    () => {
        # [doc = " An identifier used to reference a COMDAT section group."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct ComdatId (usize) ;
    };
}

ComdatId!();
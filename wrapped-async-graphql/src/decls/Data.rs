macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! Data {
    () => {
        deps!();
        # [doc = " Schema/Context data."] # [doc = ""] # [doc = " This is a type map, allowing you to store anything inside it."] # [derive (Default)] pub struct Data (FnvHashMap < TypeId , Box < dyn Any + Sync + Send > >) ;
    };
}

Data!()
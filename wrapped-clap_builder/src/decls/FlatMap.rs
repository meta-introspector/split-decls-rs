macro_rules! FlatMap {
    () => {
        # [doc = " Flat (Vec) backed map"] # [doc = ""] # [doc = " This preserves insertion order"] # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct FlatMap < K , V > { keys : Vec < K > , values : Vec < V > , }
    };
}

FlatMap!();
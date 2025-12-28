macro_rules! DropGlue {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] pub enum DropGlue { None , # [doc = " May have a drop glue if some type parameter has it."] # [doc = ""] # [doc = " For the compiler this is considered as a positive result, IDE distinguishes this from \"yes\"."] DependOnParams , HasDropGlue , }
    };
}

DropGlue!()
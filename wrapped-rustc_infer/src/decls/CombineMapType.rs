macro_rules! CombineMapType {
    () => {
        # [derive (Copy , Clone , PartialEq)] pub (crate) enum CombineMapType { Lub , Glb , }
    };
}

CombineMapType!()
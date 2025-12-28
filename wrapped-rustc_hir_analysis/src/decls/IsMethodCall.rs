macro_rules! IsMethodCall {
    () => {
        # [derive (Copy , Clone , PartialEq)] pub enum IsMethodCall { Yes , No , }
    };
}

IsMethodCall!()
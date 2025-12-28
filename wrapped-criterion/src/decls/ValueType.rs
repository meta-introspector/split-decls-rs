macro_rules! ValueType {
    () => {
        # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ValueType { Bytes , Elements , Bits , Value , }
    };
}

ValueType!();
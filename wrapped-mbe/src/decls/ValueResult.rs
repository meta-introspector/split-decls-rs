macro_rules! ValueResult {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct ValueResult < T , E > { pub value : T , pub err : Option < E > , }
    };
}

ValueResult!();
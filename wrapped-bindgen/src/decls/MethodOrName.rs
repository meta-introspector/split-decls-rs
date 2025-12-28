macro_rules! deps {
    () => {
        Method!();
    };
}

macro_rules! MethodOrName {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub enum MethodOrName { Method (Method) , Name (MethodDef) , }
    };
}

MethodOrName!()
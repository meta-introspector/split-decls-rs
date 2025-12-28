macro_rules! deps {
    () => {
        CppMethod!();
        Method!();
    };
}

macro_rules! CppMethodOrName {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub enum CppMethodOrName { Method (CppMethod) , Name (MethodDef) , }
    };
}

CppMethodOrName!();
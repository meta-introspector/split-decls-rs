macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! SYSTEM_SCALARS {
    () => {
        deps!();
        const SYSTEM_SCALARS : & [& str] = & ["Int" , "Float" , "String" , "Boolean" , "ID"] ;
    };
}

SYSTEM_SCALARS!()
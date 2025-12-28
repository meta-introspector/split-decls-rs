macro_rules! ProtocolError {
    () => {
        # [derive (Debug , Clone , PartialEq)] pub struct ProtocolError (String , bool) ;
    };
}

ProtocolError!();
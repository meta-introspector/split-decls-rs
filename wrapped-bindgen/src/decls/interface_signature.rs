macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! interface_signature {
    () => {
        deps!();
        pub fn interface_signature (def : TypeDef , generics : & [Type]) -> String { if generics . is_empty () { let guid = def . guid_attribute () . unwrap () ; format ! ("{{{guid}}}") } else { let guid = def . guid_attribute () . unwrap () ; let mut signature = format ! ("pinterface({{{guid}}}") ; for generic in generics { signature . push (';') ; signature . push_str (& generic . runtime_signature ()) } signature . push (')') ; signature } }
    };
}

interface_signature!()
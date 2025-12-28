macro_rules! deps {
    () => {
        WriteBackendMethods!();
        ThinModule!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < B : WriteBackendMethods > ThinModule < B > { pub fn name (& self) -> & str { self . shared . module_names [self . idx] . to_str () . unwrap () } pub fn cost (& self) -> u64 { self . data () . len () as u64 } pub fn data (& self) -> & [u8] { let a = self . shared . thin_buffers . get (self . idx) . map (| b | b . data ()) ; a . unwrap_or_else (| | { let len = self . shared . thin_buffers . len () ; self . shared . serialized_modules [self . idx - len] . data () }) } }
    };
}

impl_144!();
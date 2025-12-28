macro_rules! deps {
    () => {
        Result!();
        Error!();
        ResolveState!();
    };
}

macro_rules! impl_539 {
    () => {
        deps!();
        impl Serialize for ResolveState { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { let mut map = serializer . serialize_map (None) ? ; map . serialize_entry ("path" , & self . path) ? ; map . serialize_entry ("fieldName" , & self . field_name) ? ; map . serialize_entry ("parentType" , & self . parent_type) ? ; map . serialize_entry ("returnType" , & self . return_type) ? ; map . serialize_entry ("startOffset" , & self . start_offset) ? ; map . serialize_entry ("duration" , & (self . end_time - self . start_time) . num_nanoseconds () ,) ? ; map . end () } }
    };
}

impl_539!()
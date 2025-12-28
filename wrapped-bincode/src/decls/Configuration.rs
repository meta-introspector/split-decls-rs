macro_rules! deps {
    () => {
        LittleEndian!();
        Varint!();
        NoLimit!();
        Config!();
    };
}

macro_rules! Configuration {
    () => {
        deps!();
        # [doc = " The Configuration struct is used to build bincode configurations. The [Config] trait is implemented"] # [doc = " by this struct when a valid configuration has been constructed."] # [doc = ""] # [doc = " The following methods are mutually exclusive and will overwrite each other. The last call to one of these methods determines the behavior of the configuration:"] # [doc = ""] # [doc = " - [with_little_endian] and [with_big_endian]"] # [doc = " - [with_fixed_int_encoding] and [with_variable_int_encoding]"] # [doc = ""] # [doc = ""] # [doc = " [with_little_endian]: #method.with_little_endian"] # [doc = " [with_big_endian]: #method.with_big_endian"] # [doc = " [with_fixed_int_encoding]: #method.with_fixed_int_encoding"] # [doc = " [with_variable_int_encoding]: #method.with_variable_int_encoding"] # [derive (Copy , Clone , Debug)] pub struct Configuration < E = LittleEndian , I = Varint , L = NoLimit > { _e : PhantomData < E > , _i : PhantomData < I > , _l : PhantomData < L > , }
    };
}

Configuration!()
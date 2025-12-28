macro_rules! deps {
    () => {
        Error!();
        OdbWriter!();
        Oid!();
        Binding!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        impl < 'repo > OdbWriter < 'repo > { # [doc = " Finish writing to an ODB stream"] # [doc = ""] # [doc = " This method can be used to finalize writing object to the database and get an identifier."] # [doc = " The object will take its final name and will be available to the odb."] # [doc = " This method will fail if the total number of received bytes differs from the size declared with odb_writer()"] # [doc = " Attempting write after finishing will be ignored."] pub fn finalize (& mut self) -> Result < Oid , Error > { let mut raw = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; unsafe { try_call ! (raw :: git_odb_stream_finalize_write (& mut raw , self . raw)) ; Ok (Binding :: from_raw (& raw as * const _)) } } }
    };
}

impl_503!();
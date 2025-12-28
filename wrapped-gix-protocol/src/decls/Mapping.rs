macro_rules! deps {
    () => {
        Source!();
        SpecIndex!();
    };
}

macro_rules! Mapping {
    () => {
        deps!();
        # [doc = " A mapping between a single remote reference and its advertised objects to a local destination which may or may not exist."] # [derive (Debug , Clone)] pub struct Mapping { # [doc = " The reference on the remote side, along with information about the objects they point to as advertised by the server."] pub remote : Source , # [doc = " The local tracking reference to update after fetching the object visible via `remote`."] pub local : Option < bstr :: BString > , # [doc = " The index into the fetch ref-specs used to produce the mapping, allowing it to be recovered."] pub spec_index : SpecIndex , }
    };
}

Mapping!();
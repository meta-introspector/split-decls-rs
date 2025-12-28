macro_rules! OriginalHeaderOrder {
    () => {
        # [cfg (feature = "ffi")] # [derive (Clone , Debug)] # [doc = " Hashmap<Headername, numheaders with that name>"] pub (crate) struct OriginalHeaderOrder { # [doc = " Stores how many entries a Headername maps to. This is used"] # [doc = " for accounting."] num_entries : HashMap < HeaderName , usize > , # [doc = " Stores the ordering of the headers. ex: `vec[i] = (headerName, idx)`,"] # [doc = " The vector is ordered such that the ith element"] # [doc = " represents the ith header that came in off the line."] # [doc = " The `HeaderName` and `idx` are then used elsewhere to index into"] # [doc = " the multi map that stores the header values."] entry_order : Vec < (HeaderName , usize) > , }
    };
}

OriginalHeaderOrder!();
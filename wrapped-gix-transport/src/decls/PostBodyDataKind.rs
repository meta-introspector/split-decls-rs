macro_rules! PostBodyDataKind {
    () => {
        # [doc = " Whether or not the post body is expected to fit into memory or not."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum PostBodyDataKind { # [doc = " We know how much data we are sending and think it will fit into memory. This allows to collect it into a buffer"] # [doc = " and send it with `Content-Length: <body-len>`."] BoundedAndFitsIntoMemory , # [doc = " We don't know how much data we will send and assume it won't fit into memory. This enables streaming mode."] Unbounded , }
    };
}

PostBodyDataKind!();
macro_rules! Tags {
    () => {
        # [doc = " Describe how to handle tags when fetching"] # [derive (Default , Debug , Copy , Clone , PartialEq , Eq)] pub enum Tags { # [doc = " Fetch all tags from the remote, even if these are not reachable from objects referred to by our refspecs."] All , # [doc = " Fetch only the tags that point to the objects being sent."] # [doc = " That way, annotated tags that point to an object we receive are automatically transmitted and their refs are created."] # [doc = " The same goes for lightweight tags."] # [default] Included , # [doc = " Do not fetch any tags."] None , }
    };
}

Tags!();
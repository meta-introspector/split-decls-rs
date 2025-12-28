macro_rules! deps {
    () => {
        Pattern!();
        Mapping!();
    };
}

macro_rules! List {
    () => {
        deps!();
        # [doc = " A list of patterns which optionally know where they were loaded from and what their base is."] # [doc = ""] # [doc = " Knowing their base which is relative to a source directory, it will ignore all path to match against"] # [doc = " that don't also start with said base."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Default)] pub struct List < T : Pattern > { # [doc = " Patterns and their associated data in the order they were loaded in or specified,"] # [doc = " the line number in its source file or its sequence number (_`(pattern, value, line_number)`_)."] # [doc = ""] # [doc = " During matching, this order is reversed."] pub patterns : Vec < Mapping < T :: Value > > , # [doc = " The path from which the patterns were read, or `None` if the patterns"] # [doc = " don't originate in a file on disk."] pub source : Option < PathBuf > , # [doc = " The parent directory of source, or `None` if the patterns are _global_ to match against the repository root."] # [doc = " It's processed to contain slashes only and to end with a trailing slash, and is relative to the repository root."] pub base : Option < BString > , }
    };
}

List!()
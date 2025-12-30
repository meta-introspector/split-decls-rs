// Generated macro for BlameEntry (struct)
macro_rules! Depcrate_typesBlameEntry {
() => {
// Module: crate::types
// Provides: {"BlameEntry"}
// Dependencies: {}
# [doc = " A mapping of a section of the *Blamed File* to the section in a *Source File* that introduced it."] # [doc = ""] # [doc = " Both ranges are of the same size, but may use different [starting points](Range::start). Naturally,"] # [doc = " they have the same content, which is the reason they are in what is returned by [`file()`](crate::file())."] # [derive (Clone , Debug , PartialEq)] pub struct BlameEntry { # [doc = " The index of the token in the *Blamed File* (typically lines) where this entry begins."] pub start_in_blamed_file : u32 , # [doc = " The index of the token in the *Source File* (typically lines) where this entry begins."] # [doc = ""] # [doc = " This is possibly offset compared to `start_in_blamed_file`."] pub start_in_source_file : u32 , # [doc = " The amount of lines the hunk is spanning."] pub len : NonZeroU32 , # [doc = " The commit that introduced the section into the *Source File*."] pub commit_id : ObjectId , # [doc = " The *Source File*'s name, in case it differs from *Blamed File*'s name."] # [doc = " This happens when the file was renamed."] pub source_file_name : Option < BString > , }
};
}

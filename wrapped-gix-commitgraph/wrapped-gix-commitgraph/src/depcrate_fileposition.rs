// Generated macro for Position (struct)
macro_rules! Depcrate_filePosition {
() => {
// Module: crate::file
// Provides: {"Position"}
// Dependencies: {}
# [doc = " The position of a given commit within a graph file, starting at 0."] # [doc = ""] # [doc = " Commits within a graph file are sorted in lexicographical order by OID; a commit's lexicographical position"] # [doc = " is its position in this ordering. If a commit graph spans multiple files, each file's commits"] # [doc = " start at lexicographical position 0, so it is unique across a single file but is not unique across"] # [doc = " the whole commit graph. Each commit also has a graph position ([`Position`][crate::Position]),"] # [doc = " which is unique across the whole commit graph."] # [doc = " In order to avoid accidentally mixing lexicographical positions with graph positions, distinct types are used for each."] # [derive (Clone , Copy , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct Position (pub u32) ;
};
}

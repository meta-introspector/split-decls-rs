// Generated macro for DirEntryAttr (enum)
macro_rules! Depcrate_dirDirEntryAttr {
() => {
// Module: crate::dir
// Provides: {"DirEntryAttr"}
// Dependencies: {}
# [doc = " Available attributes for get information about directory entry."] # [derive (Hash , Eq , PartialEq , Clone)] pub enum DirEntryAttr { # [doc = " Folder name or file name without extension."] Name , # [doc = " File extension."] Ext , # [doc = " Folder name or file name with extension."] FullName , # [doc = " Path to file or directory."] Path , # [doc = " Dos path to file or directory."] DosPath , # [doc = " File size in bytes."] FileSize , # [doc = " Size file or directory in bytes."] # [doc = ""] # [doc = " `Attention!`: This operation very expensive and sometimes required additional rights."] Size , # [doc = " Return whether entry is directory or not."] IsDir , # [doc = " Return whether entry is file or not."] IsFile , # [doc = " Last modification time for directory entry."] Modified , # [doc = " Last access time for directory entry."] Accessed , # [doc = " Created time for directory entry."] # [doc = ""] # [doc = " `Attention!`: Not supported UNIX platform."] Created , # [doc = " Return or not return base information target folder."] BaseInfo , }
};
}

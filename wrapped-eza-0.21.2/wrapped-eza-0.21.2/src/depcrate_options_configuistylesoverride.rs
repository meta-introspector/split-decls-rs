// Generated macro for UiStylesOverride (struct)
macro_rules! Depcrate_options_configUiStylesOverride {
() => {
// Module: crate::options::config
// Provides: {"UiStylesOverride"}
// Dependencies: {}
# [rustfmt :: skip] # [derive (Debug , PartialEq , Eq , Clone , Serialize , Deserialize)] pub struct UiStylesOverride { pub colourful : Option < bool > , pub filekinds : Option < FileKindsOverride > , pub perms : Option < PermissionsOverride > , pub size : Option < SizeOverride > , pub users : Option < UsersOverride > , pub links : Option < LinksOverride > , pub git : Option < GitOverride > , pub git_repo : Option < GitRepoOverride > , pub security_context : Option < SecurityContextOverride > , pub file_type : Option < FileTypeOverride > , pub punctuation : Option < StyleOverride > , pub date : Option < StyleOverride > , pub inode : Option < StyleOverride > , pub blocks : Option < StyleOverride > , pub header : Option < StyleOverride > , pub octal : Option < StyleOverride > , pub flags : Option < StyleOverride > , pub symlink_path : Option < StyleOverride > , pub control_char : Option < StyleOverride > , pub broken_symlink : Option < StyleOverride > , pub broken_path_overlay : Option < StyleOverride > , pub filenames : Option < HashMap < String , FileNameStyleOverride > > , pub extensions : Option < HashMap < String , FileNameStyleOverride > > , }
};
}

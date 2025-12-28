macro_rules! deps {
    () => {
        SubmoduleStatus!();
        Conflict!();
        Summary!();
        Change!();
        EntryStatus!();
        Entry!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        # [doc = " Access"] impl < ContentChange , SubmoduleStatus > Entry < '_ , ContentChange , SubmoduleStatus > { # [doc = " Return a summary of the entry as digest of its status, or `None` if this entry is"] # [doc = " created from the directory walk and is *not untracked*, or if it is merely to communicate"] # [doc = " a needed update to the index entry."] pub fn summary (& self) -> Option < Summary > { Some (match self { Entry :: Modification { status : EntryStatus :: Conflict { .. } , .. } => Summary :: Conflict , Entry :: Modification { status : EntryStatus :: IntentToAdd , .. } => Summary :: IntentToAdd , Entry :: Modification { status : EntryStatus :: NeedsUpdate (_) , .. } => return None , Entry :: Modification { status : EntryStatus :: Change (change) , .. } => match change { Change :: SubmoduleModification (_) | Change :: Modification { .. } => Summary :: Modified , Change :: Type { .. } => Summary :: TypeChange , Change :: Removed => Summary :: Removed , } , Entry :: DirectoryContents { entry , .. } => { if matches ! (entry . status , gix_dir :: entry :: Status :: Untracked) { Summary :: Added } else { return None ; } } Entry :: Rewrite { copy , .. } => { if * copy { Summary :: Copied } else { Summary :: Renamed } } }) } # [doc = " The repository-relative path at which the source of a rewrite is located."] # [doc = ""] # [doc = " If this isn't a rewrite, the path is the location of the entry itself."] pub fn source_rela_path (& self) -> & BStr { match self { Entry :: Modification { rela_path , .. } => rela_path , Entry :: DirectoryContents { entry , .. } => entry . rela_path . as_bstr () , Entry :: Rewrite { source , .. } => source . rela_path () , } } # [doc = " The repository-relative path at which the destination of a rewrite is located."] # [doc = ""] # [doc = " If this isn't a rewrite, the path is the location of the entry itself."] pub fn destination_rela_path (& self) -> & BStr { match self { Entry :: Modification { rela_path , .. } => rela_path , Entry :: DirectoryContents { entry , .. } => entry . rela_path . as_bstr () , Entry :: Rewrite { dirwalk_entry , .. } => dirwalk_entry . rela_path . as_bstr () , } } }
    };
}

impl_46!()
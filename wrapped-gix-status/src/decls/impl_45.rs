macro_rules! deps {
    () => {
        SubmoduleStatus!();
        RewriteSource!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        # [doc = " Access"] impl < ContentChange , SubmoduleStatus > RewriteSource < '_ , ContentChange , SubmoduleStatus > { # [doc = " The repository-relative path of this source."] pub fn rela_path (& self) -> & BStr { match self { RewriteSource :: RewriteFromIndex { source_rela_path , .. } => source_rela_path , RewriteSource :: CopyFromDirectoryEntry { source_dirwalk_entry , .. } => source_dirwalk_entry . rela_path . as_bstr () , } } }
    };
}

impl_45!()
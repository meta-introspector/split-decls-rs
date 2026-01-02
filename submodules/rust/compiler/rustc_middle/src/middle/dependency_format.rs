mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_hir :: def_id :: CrateNum ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable } ;}
mkuse!{use rustc_session :: config :: CrateType ;}
mkitem!{# [doc = " A list of dependencies for a certain crate type."] pub type DependencyList = IndexVec < CrateNum , Linkage > ;}
mkitem!{# [doc = " A mapping of all required dependencies for a particular flavor of output."] # [doc = ""] # [doc = " This is local to the tcx, and is generally relevant to one session."] pub type Dependencies = FxIndexMap < CrateType , DependencyList > ;}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Debug , HashStable , Encodable , Decodable)] pub enum Linkage { NotLinked , IncludedFromDylib , Static , Dynamic , }}}
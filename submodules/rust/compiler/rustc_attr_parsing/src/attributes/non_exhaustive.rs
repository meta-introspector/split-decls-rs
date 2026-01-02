mkuse!{use rustc_hir :: Target ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use crate :: attributes :: { NoArgsAttributeParser , OnDuplicate } ;}
mkuse!{use crate :: context :: Stage ;}
mkuse!{use crate :: target_checking :: AllowedTargets ;}
mkuse!{use crate :: target_checking :: Policy :: { Allow , Warn } ;}
mkitem!{mkstruct!{pub (crate) struct NonExhaustiveParser ;}}
mkitem!{mkimpl!{impl < S : Stage > NoArgsAttributeParser < S > for NonExhaustiveParser { const PATH : & [Symbol] = & [sym :: non_exhaustive] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Enum) , Allow (Target :: Struct) , Allow (Target :: Variant) , Warn (Target :: Field) , Warn (Target :: Arm) , Warn (Target :: MacroDef) , Warn (Target :: MacroCall) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NonExhaustive ; }}}
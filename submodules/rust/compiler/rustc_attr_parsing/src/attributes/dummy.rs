mkuse!{use rustc_feature :: { AttributeTemplate , template } ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_span :: { Symbol , sym } ;}
mkuse!{use crate :: attributes :: { AttributeOrder , OnDuplicate , SingleAttributeParser } ;}
mkuse!{use crate :: context :: { AcceptContext , Stage } ;}
mkuse!{use crate :: parser :: ArgParser ;}
mkuse!{use crate :: target_checking :: { ALL_TARGETS , AllowedTargets } ;}
mkitem!{mkstruct!{pub (crate) struct DummyParser ;}}
mkitem!{mkimpl!{impl < S : Stage > SingleAttributeParser < S > for DummyParser { const PATH : & [Symbol] = & [sym :: rustc_dummy] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Ignore ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const TEMPLATE : AttributeTemplate = template ! (Word) ; fn convert (_ : & mut AcceptContext < '_ , '_ , S > , _ : & ArgParser < '_ >) -> Option < AttributeKind > { Some (AttributeKind :: Dummy) } }}}
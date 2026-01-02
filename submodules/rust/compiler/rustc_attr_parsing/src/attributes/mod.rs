mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use rustc_feature :: { AttributeTemplate , AttributeType , template } ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_span :: { Span , Symbol } ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use crate :: context :: { AcceptContext , FinalizeContext , Stage } ;}
mkuse!{use crate :: parser :: ArgParser ;}
mkuse!{use crate :: session_diagnostics :: UnusedMultiple ;}
mkuse!{use crate :: target_checking :: AllowedTargets ;}
mkmod!{prelude, { 
                getname!(prelude);
                getsrc!(prelude);
                getpath!(prelude);
                get_deps!(prelude);
                get_crates!(prelude);
                mkinclude!(prelude);
                 
            }}
mkmod!{allow_unstable, { 
                getname!(allow_unstable);
                getsrc!(allow_unstable);
                getpath!(allow_unstable);
                get_deps!(allow_unstable);
                get_crates!(allow_unstable);
                mkinclude!(allow_unstable);
                 
            }}
mkmod!{body, { 
                getname!(body);
                getsrc!(body);
                getpath!(body);
                get_deps!(body);
                get_crates!(body);
                mkinclude!(body);
                 
            }}
mkmod!{cfg, { 
                getname!(cfg);
                getsrc!(cfg);
                getpath!(cfg);
                get_deps!(cfg);
                get_crates!(cfg);
                mkinclude!(cfg);
                 
            }}
mkmod!{cfg_old, { 
                getname!(cfg_old);
                getsrc!(cfg_old);
                getpath!(cfg_old);
                get_deps!(cfg_old);
                get_crates!(cfg_old);
                mkinclude!(cfg_old);
                 
            }}
mkmod!{codegen_attrs, { 
                getname!(codegen_attrs);
                getsrc!(codegen_attrs);
                getpath!(codegen_attrs);
                get_deps!(codegen_attrs);
                get_crates!(codegen_attrs);
                mkinclude!(codegen_attrs);
                 
            }}
mkmod!{confusables, { 
                getname!(confusables);
                getsrc!(confusables);
                getpath!(confusables);
                get_deps!(confusables);
                get_crates!(confusables);
                mkinclude!(confusables);
                 
            }}
mkmod!{crate_level, { 
                getname!(crate_level);
                getsrc!(crate_level);
                getpath!(crate_level);
                get_deps!(crate_level);
                get_crates!(crate_level);
                mkinclude!(crate_level);
                 
            }}
mkmod!{deprecation, { 
                getname!(deprecation);
                getsrc!(deprecation);
                getpath!(deprecation);
                get_deps!(deprecation);
                get_crates!(deprecation);
                mkinclude!(deprecation);
                 
            }}
mkmod!{dummy, { 
                getname!(dummy);
                getsrc!(dummy);
                getpath!(dummy);
                get_deps!(dummy);
                get_crates!(dummy);
                mkinclude!(dummy);
                 
            }}
mkmod!{inline, { 
                getname!(inline);
                getsrc!(inline);
                getpath!(inline);
                get_deps!(inline);
                get_crates!(inline);
                mkinclude!(inline);
                 
            }}
mkmod!{link_attrs, { 
                getname!(link_attrs);
                getsrc!(link_attrs);
                getpath!(link_attrs);
                get_deps!(link_attrs);
                get_crates!(link_attrs);
                mkinclude!(link_attrs);
                 
            }}
mkmod!{lint_helpers, { 
                getname!(lint_helpers);
                getsrc!(lint_helpers);
                getpath!(lint_helpers);
                get_deps!(lint_helpers);
                get_crates!(lint_helpers);
                mkinclude!(lint_helpers);
                 
            }}
mkmod!{loop_match, { 
                getname!(loop_match);
                getsrc!(loop_match);
                getpath!(loop_match);
                get_deps!(loop_match);
                get_crates!(loop_match);
                mkinclude!(loop_match);
                 
            }}
mkmod!{macro_attrs, { 
                getname!(macro_attrs);
                getsrc!(macro_attrs);
                getpath!(macro_attrs);
                get_deps!(macro_attrs);
                get_crates!(macro_attrs);
                mkinclude!(macro_attrs);
                 
            }}
mkmod!{must_use, { 
                getname!(must_use);
                getsrc!(must_use);
                getpath!(must_use);
                get_deps!(must_use);
                get_crates!(must_use);
                mkinclude!(must_use);
                 
            }}
mkmod!{no_implicit_prelude, { 
                getname!(no_implicit_prelude);
                getsrc!(no_implicit_prelude);
                getpath!(no_implicit_prelude);
                get_deps!(no_implicit_prelude);
                get_crates!(no_implicit_prelude);
                mkinclude!(no_implicit_prelude);
                 
            }}
mkmod!{non_exhaustive, { 
                getname!(non_exhaustive);
                getsrc!(non_exhaustive);
                getpath!(non_exhaustive);
                get_deps!(non_exhaustive);
                get_crates!(non_exhaustive);
                mkinclude!(non_exhaustive);
                 
            }}
mkmod!{path, { 
                getname!(path);
                getsrc!(path);
                getpath!(path);
                get_deps!(path);
                get_crates!(path);
                mkinclude!(path);
                 
            }}
mkmod!{proc_macro_attrs, { 
                getname!(proc_macro_attrs);
                getsrc!(proc_macro_attrs);
                getpath!(proc_macro_attrs);
                get_deps!(proc_macro_attrs);
                get_crates!(proc_macro_attrs);
                mkinclude!(proc_macro_attrs);
                 
            }}
mkmod!{prototype, { 
                getname!(prototype);
                getsrc!(prototype);
                getpath!(prototype);
                get_deps!(prototype);
                get_crates!(prototype);
                mkinclude!(prototype);
                 
            }}
mkmod!{repr, { 
                getname!(repr);
                getsrc!(repr);
                getpath!(repr);
                get_deps!(repr);
                get_crates!(repr);
                mkinclude!(repr);
                 
            }}
mkmod!{rustc_internal, { 
                getname!(rustc_internal);
                getsrc!(rustc_internal);
                getpath!(rustc_internal);
                get_deps!(rustc_internal);
                get_crates!(rustc_internal);
                mkinclude!(rustc_internal);
                 
            }}
mkmod!{semantics, { 
                getname!(semantics);
                getsrc!(semantics);
                getpath!(semantics);
                get_deps!(semantics);
                get_crates!(semantics);
                mkinclude!(semantics);
                 
            }}
mkmod!{stability, { 
                getname!(stability);
                getsrc!(stability);
                getpath!(stability);
                get_deps!(stability);
                get_crates!(stability);
                mkinclude!(stability);
                 
            }}
mkmod!{test_attrs, { 
                getname!(test_attrs);
                getsrc!(test_attrs);
                getpath!(test_attrs);
                get_deps!(test_attrs);
                get_crates!(test_attrs);
                mkinclude!(test_attrs);
                 
            }}
mkmod!{traits, { 
                getname!(traits);
                getsrc!(traits);
                getpath!(traits);
                get_deps!(traits);
                get_crates!(traits);
                mkinclude!(traits);
                 
            }}
mkmod!{transparency, { 
                getname!(transparency);
                getsrc!(transparency);
                getpath!(transparency);
                get_deps!(transparency);
                get_crates!(transparency);
                mkinclude!(transparency);
                 
            }}
mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                 
            }}
mkitem!{type AcceptFn < T , S > = for < 'sess > fn (& mut T , & mut AcceptContext < '_ , 'sess , S > , & ArgParser < '_ >) ;}
mkitem!{type AcceptMapping < T , S > = & 'static [(& 'static [Symbol] , AttributeTemplate , AcceptFn < T , S >)] ;}
mkitem!{mktrait!{# [doc = " An [`AttributeParser`] is a type which searches for syntactic attributes."] # [doc = ""] # [doc = " Parsers are often tiny state machines that gets to see all syntactical attributes on an item."] # [doc = " [`Default::default`] creates a fresh instance that sits in some kind of initial state, usually that the"] # [doc = " attribute it is looking for was not yet seen."] # [doc = ""] # [doc = " Then, it defines what paths this group will accept in [`AttributeParser::ATTRIBUTES`]."] # [doc = " These are listed as pairs, of symbols and function pointers. The function pointer will"] # [doc = " be called when that attribute is found on an item, which can influence the state of the little"] # [doc = " state machine."] # [doc = ""] # [doc = " Finally, after all attributes on an item have been seen, and possibly been accepted,"] # [doc = " the [`finalize`](AttributeParser::finalize) functions for all attribute parsers are called. Each can then report"] # [doc = " whether it has seen the attribute it has been looking for."] # [doc = ""] # [doc = " The state machine is automatically reset to parse attributes on the next item."] # [doc = ""] # [doc = " For a simpler attribute parsing interface, consider using [`SingleAttributeParser`]"] # [doc = " or [`CombineAttributeParser`] instead."] pub (crate) trait AttributeParser < S : Stage > : Default + 'static { # [doc = " The symbols for the attributes that this parser is interested in."] # [doc = ""] # [doc = " If an attribute has this symbol, the `accept` function will be called on it."] const ATTRIBUTES : AcceptMapping < Self , S > ; const ALLOWED_TARGETS : AllowedTargets ; const TYPE : AttributeType = AttributeType :: Normal ; # [doc = " The parser has gotten a chance to accept the attributes on an item,"] # [doc = " here it can produce an attribute."] # [doc = ""] # [doc = " All finalize methods of all parsers are unconditionally called."] # [doc = " This means you can't unconditionally return `Some` here,"] # [doc = " that'd be equivalent to unconditionally applying an attribute to"] # [doc = " every single syntax item that could have attributes applied to it."] # [doc = " Your accept mappings should determine whether this returns something."] fn finalize (self , cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > ; }}}
mkitem!{mktrait!{# [doc = " Alternative to [`AttributeParser`] that automatically handles state management."] # [doc = " A slightly simpler and more restricted way to convert attributes."] # [doc = " Assumes that an attribute can only appear a single time on an item,"] # [doc = " and errors when it sees more."] # [doc = ""] # [doc = " [`Single<T> where T: SingleAttributeParser`](Single) implements [`AttributeParser`]."] # [doc = ""] # [doc = " [`SingleAttributeParser`] can only convert attributes one-to-one, and cannot combine multiple"] # [doc = " attributes together like is necessary for `#[stable()]` and `#[unstable()]` for example."] pub (crate) trait SingleAttributeParser < S : Stage > : 'static { # [doc = " The single path of the attribute this parser accepts."] # [doc = ""] # [doc = " If you need the parser to accept more than one path, use [`AttributeParser`] instead"] const PATH : & [Symbol] ; # [doc = " Configures the precedence of attributes with the same `PATH` on a syntax node."] const ATTRIBUTE_ORDER : AttributeOrder ; # [doc = " Configures what to do when when the same attribute is"] # [doc = " applied more than once on the same syntax node."] # [doc = ""] # [doc = " [`ATTRIBUTE_ORDER`](Self::ATTRIBUTE_ORDER) specified which one is assumed to be correct,"] # [doc = " and this specified whether to, for example, warn or error on the other one."] const ON_DUPLICATE : OnDuplicate < S > ; const ALLOWED_TARGETS : AllowedTargets ; # [doc = " The template this attribute parser should implement. Used for diagnostics."] const TEMPLATE : AttributeTemplate ; const TYPE : AttributeType = AttributeType :: Normal ; # [doc = " Converts a single syntactical attribute to a single semantic attribute, or [`AttributeKind`]"] fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > ; }}}
mkitem!{mkstruct!{# [doc = " Use in combination with [`SingleAttributeParser`]."] # [doc = " `Single<T: SingleAttributeParser>` implements [`AttributeParser`]."] pub (crate) struct Single < T : SingleAttributeParser < S > , S : Stage > (PhantomData < (S , T) > , Option < (AttributeKind , Span) > ,) ;}}
mkitem!{mkimpl!{impl < T : SingleAttributeParser < S > , S : Stage > Default for Single < T , S > { fn default () -> Self { Self (Default :: default () , Default :: default ()) } }}}
mkitem!{mkimpl!{impl < T : SingleAttributeParser < S > , S : Stage > AttributeParser < S > for Single < T , S > { const ATTRIBUTES : AcceptMapping < Self , S > = & [(T :: PATH , < T as SingleAttributeParser < S > > :: TEMPLATE , | group : & mut Single < T , S > , cx , args | { if let Some (pa) = T :: convert (cx , args) { match T :: ATTRIBUTE_ORDER { AttributeOrder :: KeepInnermost => { if let Some ((_ , unused)) = group . 1 { T :: ON_DUPLICATE . exec :: < T > (cx , cx . attr_span , unused) ; return ; } } AttributeOrder :: KeepOutermost => { if let Some ((_ , used)) = group . 1 { T :: ON_DUPLICATE . exec :: < T > (cx , used , cx . attr_span) ; } } } group . 1 = Some ((pa , cx . attr_span)) ; } } ,)] ; const ALLOWED_TARGETS : AllowedTargets = T :: ALLOWED_TARGETS ; const TYPE : AttributeType = T :: TYPE ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { Some (self . 1 ? . 0) } }}}
mkitem!{mkenum!{pub (crate) enum OnDuplicate < S : Stage > { # [doc = " Give a default warning"] Warn , # [doc = " Duplicates will be a warning, with a note that this will be an error in the future."] WarnButFutureError , # [doc = " Give a default error"] Error , # [doc = " Ignore duplicates"] Ignore , # [doc = " Custom function called when a duplicate attribute is found."] # [doc = ""] # [doc = " - `unused` is the span of the attribute that was unused or bad because of some"] # [doc = "   duplicate reason (see [`AttributeOrder`])"] # [doc = " - `used` is the span of the attribute that was used in favor of the unused attribute"] Custom (fn (cx : & AcceptContext < '_ , '_ , S > , used : Span , unused : Span)) , }}}
mkitem!{mkimpl!{impl < S : Stage > OnDuplicate < S > { fn exec < P : SingleAttributeParser < S > > (& self , cx : & mut AcceptContext < '_ , '_ , S > , used : Span , unused : Span ,) { match self { OnDuplicate :: Warn => cx . warn_unused_duplicate (used , unused) , OnDuplicate :: WarnButFutureError => cx . warn_unused_duplicate_future_error (used , unused) , OnDuplicate :: Error => { cx . emit_err (UnusedMultiple { this : used , other : unused , name : Symbol :: intern (& P :: PATH . into_iter () . map (| i | i . to_string ()) . collect :: < Vec < _ > > () . join ("..") ,) , }) ; } OnDuplicate :: Ignore => { } OnDuplicate :: Custom (f) => f (cx , used , unused) , } } }}}
mkitem!{mkenum!{pub (crate) enum AttributeOrder { # [doc = " Duplicates after the innermost instance of the attribute will be an error/warning."] # [doc = " Only keep the lowest attribute."] # [doc = ""] # [doc = " Attributes are processed from bottom to top, so this raises a warning/error on all the attributes"] # [doc = " further above the lowest one:"] # [doc = " ```"] # [doc = " #[stable(since=\"1.0\")] //~ WARNING duplicated attribute"] # [doc = " #[stable(since=\"2.0\")]"] # [doc = " ```"] KeepInnermost , # [doc = " Duplicates before the outermost instance of the attribute will be an error/warning."] # [doc = " Only keep the highest attribute."] # [doc = ""] # [doc = " Attributes are processed from bottom to top, so this raises a warning/error on all the attributes"] # [doc = " below the highest one:"] # [doc = " ```"] # [doc = " #[path=\"foo.rs\"]"] # [doc = " #[path=\"bar.rs\"] //~ WARNING duplicated attribute"] # [doc = " ```"] KeepOutermost , }}}
mkitem!{mktrait!{# [doc = " An even simpler version of [`SingleAttributeParser`]:"] # [doc = " now automatically check that there are no arguments provided to the attribute."] # [doc = ""] # [doc = " [`WithoutArgs<T> where T: NoArgsAttributeParser`](WithoutArgs) implements [`SingleAttributeParser`]."] pub (crate) trait NoArgsAttributeParser < S : Stage > : 'static { const PATH : & [Symbol] ; const ON_DUPLICATE : OnDuplicate < S > ; const ALLOWED_TARGETS : AllowedTargets ; const TYPE : AttributeType = AttributeType :: Normal ; # [doc = " Create the [`AttributeKind`] given attribute's [`Span`]."] const CREATE : fn (Span) -> AttributeKind ; }}}
mkitem!{mkstruct!{pub (crate) struct WithoutArgs < T : NoArgsAttributeParser < S > , S : Stage > (PhantomData < (S , T) >) ;}}
mkitem!{mkimpl!{impl < T : NoArgsAttributeParser < S > , S : Stage > Default for WithoutArgs < T , S > { fn default () -> Self { Self (Default :: default ()) } }}}
mkitem!{mkimpl!{impl < T : NoArgsAttributeParser < S > , S : Stage > SingleAttributeParser < S > for WithoutArgs < T , S > { const PATH : & [Symbol] = T :: PATH ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = T :: ON_DUPLICATE ; const ALLOWED_TARGETS : AllowedTargets = T :: ALLOWED_TARGETS ; const TEMPLATE : AttributeTemplate = template ! (Word) ; const TYPE : AttributeType = T :: TYPE ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { if let Err (span) = args . no_args () { cx . expected_no_args (span) ; } Some (T :: CREATE (cx . attr_span)) } }}}
mkitem!{type ConvertFn < E > = fn (ThinVec < E > , Span) -> AttributeKind ;}
mkitem!{mktrait!{# [doc = " Alternative to [`AttributeParser`] that automatically handles state management."] # [doc = " If multiple attributes appear on an element, combines the values of each into a"] # [doc = " [`ThinVec`]."] # [doc = " [`Combine<T> where T: CombineAttributeParser`](Combine) implements [`AttributeParser`]."] # [doc = ""] # [doc = " [`CombineAttributeParser`] can only convert a single kind of attribute, and cannot combine multiple"] # [doc = " attributes together like is necessary for `#[stable()]` and `#[unstable()]` for example."] pub (crate) trait CombineAttributeParser < S : Stage > : 'static { const PATH : & [rustc_span :: Symbol] ; type Item ; # [doc = " A function that converts individual items (of type [`Item`](Self::Item)) into the final attribute."] # [doc = ""] # [doc = " For example, individual representations fomr `#[repr(...)]` attributes into an `AttributeKind::Repr(x)`,"] # [doc = "  where `x` is a vec of these individual reprs."] const CONVERT : ConvertFn < Self :: Item > ; const ALLOWED_TARGETS : AllowedTargets ; # [doc = " The template this attribute parser should implement. Used for diagnostics."] const TEMPLATE : AttributeTemplate ; const TYPE : AttributeType = AttributeType :: Normal ; # [doc = " Converts a single syntactical attribute to a number of elements of the semantic attribute, or [`AttributeKind`]"] fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > + 'c ; }}}
mkitem!{mkstruct!{# [doc = " Use in combination with [`CombineAttributeParser`]."] # [doc = " `Combine<T: CombineAttributeParser>` implements [`AttributeParser`]."] pub (crate) struct Combine < T : CombineAttributeParser < S > , S : Stage > { phantom : PhantomData < (S , T) > , # [doc = " A list of all items produced by parsing attributes so far. One attribute can produce any amount of items."] items : ThinVec < < T as CombineAttributeParser < S > > :: Item > , # [doc = " The full span of the first attribute that was encountered."] first_span : Option < Span > , }}}
mkitem!{mkimpl!{impl < T : CombineAttributeParser < S > , S : Stage > Default for Combine < T , S > { fn default () -> Self { Self { phantom : Default :: default () , items : Default :: default () , first_span : Default :: default () , } } }}}
mkitem!{mkimpl!{impl < T : CombineAttributeParser < S > , S : Stage > AttributeParser < S > for Combine < T , S > { const ATTRIBUTES : AcceptMapping < Self , S > = & [(T :: PATH , T :: TEMPLATE , | group : & mut Combine < T , S > , cx , args | { group . first_span . get_or_insert (cx . attr_span) ; group . items . extend (T :: extend (cx , args)) })] ; const ALLOWED_TARGETS : AllowedTargets = T :: ALLOWED_TARGETS ; const TYPE : AttributeType = T :: TYPE ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { if let Some (first_span) = self . first_span { Some (T :: CONVERT (self . items , first_span)) } else { None } } }}}
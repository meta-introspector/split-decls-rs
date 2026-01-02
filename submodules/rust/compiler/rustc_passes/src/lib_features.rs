mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: intravisit :: Visitor ;}
mkuse!{use rustc_hir :: { Attribute , StabilityLevel , StableSince } ;}
mkuse!{use rustc_middle :: hir :: nested_filter ;}
mkuse!{use rustc_middle :: middle :: lib_features :: { FeatureStability , LibFeatures } ;}
mkuse!{use rustc_middle :: query :: { LocalCrate , Providers } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use crate :: errors :: { FeaturePreviouslyDeclared , FeatureStableTwice } ;}
mkitem!{mkstruct!{struct LibFeatureCollector < 'tcx > { tcx : TyCtxt < 'tcx > , lib_features : LibFeatures , }}}
mkitem!{mkimpl!{impl < 'tcx > LibFeatureCollector < 'tcx > { fn new (tcx : TyCtxt < 'tcx >) -> LibFeatureCollector < 'tcx > { LibFeatureCollector { tcx , lib_features : LibFeatures :: default () } } fn extract (& self , attr : & Attribute) -> Option < (Symbol , FeatureStability , Span) > { let (feature , level , span) = match attr { Attribute :: Parsed (AttributeKind :: Stability { stability , span }) => { (stability . feature , stability . level , * span) } Attribute :: Parsed (AttributeKind :: ConstStability { stability , span }) => { (stability . feature , stability . level , * span) } Attribute :: Parsed (AttributeKind :: BodyStability { stability , span }) => { (stability . feature , stability . level , * span) } _ => return None , } ; let feature_stability = match level { StabilityLevel :: Unstable { old_name , .. } => FeatureStability :: Unstable { old_name } , StabilityLevel :: Stable { since , .. } => FeatureStability :: AcceptedSince (match since { StableSince :: Version (v) => Symbol :: intern (& v . to_string ()) , StableSince :: Current => sym :: env_CFG_RELEASE , StableSince :: Err (_) => return None , }) , } ; Some ((feature , feature_stability , span)) } fn collect_feature (& mut self , feature : Symbol , stability : FeatureStability , span : Span) { let existing_stability = self . lib_features . stability . get (& feature) . cloned () ; match (stability , existing_stability) { (_ , None) => { self . lib_features . stability . insert (feature , (stability , span)) ; } (FeatureStability :: AcceptedSince (since) , Some ((FeatureStability :: AcceptedSince (prev_since) , _)) ,) => { if prev_since != since { self . tcx . dcx () . emit_err (FeatureStableTwice { span , feature , since , prev_since , }) ; } } (FeatureStability :: AcceptedSince (_) , Some ((FeatureStability :: Unstable { .. } , _))) => { self . tcx . dcx () . emit_err (FeaturePreviouslyDeclared { span , feature , declared : "stable" , prev_declared : "unstable" , }) ; } (FeatureStability :: Unstable { .. } , Some ((FeatureStability :: AcceptedSince (_) , _))) => { self . tcx . dcx () . emit_err (FeaturePreviouslyDeclared { span , feature , declared : "unstable" , prev_declared : "stable" , }) ; } (FeatureStability :: Unstable { .. } , Some ((FeatureStability :: Unstable { .. } , _))) => { } } } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for LibFeatureCollector < 'tcx > { type NestedFilter = nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_attribute (& mut self , attr : & 'tcx Attribute) { if let Some ((feature , stable , span)) = self . extract (attr) { self . collect_feature (feature , stable , span) ; } } }}}

macro_rules! lib_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lib_features in module {}", module_path!());
    };
}

mkfn!{
    lib_features_introspect!();
    fn lib_features (tcx : TyCtxt < '_ > , LocalCrate : LocalCrate) -> LibFeatures { if ! tcx . features () . staged_api () { return LibFeatures :: default () ; } let mut collector = LibFeatureCollector :: new (tcx) ; tcx . hir_walk_attributes (& mut collector) ; collector . lib_features }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { providers . lib_features = lib_features ; }
}
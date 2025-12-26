use std::sync::Arc;
use crate::resolver_traits::OpaqueDeriveResolution;
use crate::syntax_extension_trait::SyntaxExtensionTrait;

pub type OptionArcdynSyntaxExtensionTraitDRT<DRT> = Option<Arc<dyn SyntaxExtensionTrait<DRT>>>;

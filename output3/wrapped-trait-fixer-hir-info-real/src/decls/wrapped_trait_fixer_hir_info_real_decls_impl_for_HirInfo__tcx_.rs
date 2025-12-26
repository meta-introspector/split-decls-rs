use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'tcx> HirInfo<'tcx> for RealHirInfoItem<'tcx> {
    type OwnerId = OwnerId;
    type ItemKind = ItemKind<'tcx>;
    type Span = Span;
    fn get_owner_id(&self) -> OwnerId {
        self.0.owner_id
    }
    fn get_item_kind<'a>(&'a self) -> &'a ItemKind<'tcx> {
        &self.0.kind
    }
    fn get_item_span(&self) -> Span {
        self.0.span
    }
}

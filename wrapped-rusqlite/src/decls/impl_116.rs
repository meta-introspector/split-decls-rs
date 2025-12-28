macro_rules! deps {
    () => {
        Action!();
        PreUpdateCase!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl From < PreUpdateCase > for Action { fn from (puc : PreUpdateCase) -> Action { match puc { PreUpdateCase :: Insert (_) => Action :: SQLITE_INSERT , PreUpdateCase :: Delete (_) => Action :: SQLITE_DELETE , PreUpdateCase :: Update { .. } => Action :: SQLITE_UPDATE , PreUpdateCase :: Unknown => Action :: UNKNOWN , } } }
    };
}

impl_116!()
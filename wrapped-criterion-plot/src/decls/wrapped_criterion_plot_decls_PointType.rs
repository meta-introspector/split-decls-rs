use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Point type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum PointType {
    Circle,
    FilledCircle,
    FilledSquare,
    FilledTriangle,
    Plus,
    Square,
    Star,
    Triangle,
    X,
}

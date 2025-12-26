use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "serde")]
impl<'de, T, const N: usize> Visitor<'de> for SmallVecVisitor<T, N>
where
    T: Deserialize<'de>,
{
    type Value = SmallVec<T, N>;
    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("a sequence")
    }
    fn visit_seq<B>(self, mut seq: B) -> Result<Self::Value, B::Error>
    where
        B: SeqAccess<'de>,
    {
        use serde_core::de::Error;
        let len = seq.size_hint().unwrap_or(0);
        let mut values = SmallVec::new();
        values.try_reserve(len).map_err(B::Error::custom)?;
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(values)
    }
}

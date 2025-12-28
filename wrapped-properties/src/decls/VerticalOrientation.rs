macro_rules! deps {
    () => {
        CodePointMapData!();
    };
}

macro_rules! VerticalOrientation {
    () => {
        deps!();
        # [doc = " Property Vertical_Orientation"] # [doc = ""] # [doc = " See UTR #50:"] # [doc = " <https://www.unicode.org/reports/tr50/#vo>"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::VerticalOrientation, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<VerticalOrientation>::new().get('a'),"] # [doc = "     VerticalOrientation::Rotated"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<VerticalOrientation>::new().get('§'),"] # [doc = "     VerticalOrientation::Upright"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<VerticalOrientation>::new().get32(0x2329),"] # [doc = "     VerticalOrientation::TransformedRotated"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<VerticalOrientation>::new().get32(0x3001),"] # [doc = "     VerticalOrientation::TransformedUpright"] # [doc = " );"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct VerticalOrientation (pub (crate) u8) ;
    };
}

VerticalOrientation!()
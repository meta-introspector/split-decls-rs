macro_rules! cfg_ {
    () => {
        mod cfg_ { use cfg :: CfgAtom ; use serde :: { Deserialize , Serialize } ; pub (super) fn deserialize < 'de , D > (deserializer : D) -> Result < Vec < CfgAtom > , D :: Error > where D : serde :: Deserializer < 'de > , { let cfg : Vec < String > = Vec :: deserialize (deserializer) ? ; cfg . into_iter () . map (| it | crate :: parse_cfg (& it) . map_err (serde :: de :: Error :: custom)) . collect () } pub (super) fn serialize < S > (cfg : & [CfgAtom] , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { cfg . iter () . map (| cfg | match cfg { CfgAtom :: Flag (flag) => flag . as_str () . to_owned () , CfgAtom :: KeyValue { key , value } => { format ! ("{}=\"{}\"" , key . as_str () , value . as_str ()) } }) . collect :: < Vec < String > > () . serialize (serializer) } }
    };
}

cfg_!();
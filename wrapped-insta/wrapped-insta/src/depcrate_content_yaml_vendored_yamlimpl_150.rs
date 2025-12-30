// Generated macro for impl_150 (impl)
macro_rules! Depcrate_content_yaml_vendored_yamlimpl_150 {
() => {
// Module: crate::content::yaml::vendored::yaml
// Provides: {"impl_150"}
// Dependencies: {}
impl YamlLoader { fn insert_new_node (& mut self , node : (Yaml , usize)) { if node . 1 > 0 { self . anchor_map . insert (node . 1 , node . 0 . clone ()) ; } if self . doc_stack . is_empty () { self . doc_stack . push (node) ; } else { let parent = self . doc_stack . last_mut () . unwrap () ; match * parent { (Yaml :: Array (ref mut v) , _) => v . push (node . 0) , (Yaml :: Hash (ref mut h) , _) => { let cur_key = self . key_stack . last_mut () . unwrap () ; if cur_key . is_badvalue () { * cur_key = node . 0 ; } else { let mut newkey = Yaml :: BadValue ; mem :: swap (& mut newkey , cur_key) ; h . push ((newkey , node . 0)) ; } } _ => unreachable ! () , } } } pub fn load_from_str (source : & str) -> Result < Vec < Yaml > , ScanError > { let mut loader = YamlLoader { docs : Vec :: new () , doc_stack : Vec :: new () , key_stack : Vec :: new () , anchor_map : BTreeMap :: new () , } ; let mut parser = Parser :: new (source . chars ()) ; parser . load (& mut loader , true) ? ; Ok (loader . docs) } }
};
}

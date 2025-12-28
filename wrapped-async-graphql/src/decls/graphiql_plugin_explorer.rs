macro_rules! deps {
    () => {
        GraphiQLPlugin!();
    };
}

macro_rules! graphiql_plugin_explorer {
    () => {
        deps!();
        # [doc = " Generate simple explorer plugin for GraphiQL (v2)"] pub fn graphiql_plugin_explorer < 'a > () -> GraphiQLPlugin < 'a > { GraphiQLPlugin { name : "GraphiQLPluginExplorer" , constructor : "GraphiQLPluginExplorer.explorerPlugin" , head_assets : Some (r#"<link rel="stylesheet" href="https://unpkg.com/@graphiql/plugin-explorer/dist/style.css" />"# ,) , body_assets : Some (r#"<script
      src="https://unpkg.com/@graphiql/plugin-explorer/dist/index.umd.js"
      crossorigin
    ></script>"# ,) , .. Default :: default () } }
    };
}

graphiql_plugin_explorer!();
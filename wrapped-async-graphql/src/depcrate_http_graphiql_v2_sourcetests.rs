// Generated macro for tests (module)
macro_rules! Depcrate_http_graphiql_v2_sourcetests {
() => {
// Module: crate::http::graphiql_v2_source
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_with_only_url () { let graphiql_source = GraphiQLSource :: build () . endpoint ("/") . finish () ; assert_eq ! (graphiql_source , r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="robots" content="noindex">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="referrer" content="origin">

    <title>GraphiQL IDE</title>

    <style>
      body {
        height: 100%;
        margin: 0;
        width: 100%;
        overflow: hidden;
      }

      #graphiql {
        height: 100vh;
      }
    </style>
    <script
      crossorigin
      src="https://unpkg.com/react@18/umd/react.development.js"
    ></script>
    <script
      crossorigin
      src="https://unpkg.com/react-dom@18/umd/react-dom.development.js"
    ></script>
    <link rel="icon" href="https://graphql.org/favicon.ico">
    <link rel="stylesheet" href="https://unpkg.com/graphiql@4/graphiql.min.css" />
  </head>

  <body>
    <div id="graphiql">Loading...</div>
    <script
      src="https://unpkg.com/graphiql@4/graphiql.min.js"
      type="application/javascript"
    ></script>
    <script>
      customFetch = (url, opts = {}) => {
        return fetch(url, {...opts, credentials: 'same-origin'})
      }

      createUrl = (endpoint, subscription = false) => {
        const url = new URL(endpoint, window.location.origin);
        if (subscription) {
          url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
        }
        return url.toString();
      }

      ReactDOM.createRoot(document.getElementById("graphiql")).render(
        React.createElement(GraphiQL, {
          fetcher: GraphiQL.createFetcher({
            url: createUrl('/'),
            fetch: customFetch,
          }),
          defaultEditorToolsVisibility: true,
        })
      );
    </script>
  </body>
</html>"#) } # [test] fn test_with_both_urls () { let graphiql_source = GraphiQLSource :: build () . endpoint ("/") . subscription_endpoint ("/ws") . finish () ; assert_eq ! (graphiql_source , r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="robots" content="noindex">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="referrer" content="origin">

    <title>GraphiQL IDE</title>

    <style>
      body {
        height: 100%;
        margin: 0;
        width: 100%;
        overflow: hidden;
      }

      #graphiql {
        height: 100vh;
      }
    </style>
    <script
      crossorigin
      src="https://unpkg.com/react@18/umd/react.development.js"
    ></script>
    <script
      crossorigin
      src="https://unpkg.com/react-dom@18/umd/react-dom.development.js"
    ></script>
    <link rel="icon" href="https://graphql.org/favicon.ico">
    <link rel="stylesheet" href="https://unpkg.com/graphiql@4/graphiql.min.css" />
  </head>

  <body>
    <div id="graphiql">Loading...</div>
    <script
      src="https://unpkg.com/graphiql@4/graphiql.min.js"
      type="application/javascript"
    ></script>
    <script>
      customFetch = (url, opts = {}) => {
        return fetch(url, {...opts, credentials: 'same-origin'})
      }

      createUrl = (endpoint, subscription = false) => {
        const url = new URL(endpoint, window.location.origin);
        if (subscription) {
          url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
        }
        return url.toString();
      }

      ReactDOM.createRoot(document.getElementById("graphiql")).render(
        React.createElement(GraphiQL, {
          fetcher: GraphiQL.createFetcher({
            url: createUrl('/'),
            fetch: customFetch,
            subscriptionUrl: createUrl('/ws', true),
          }),
          defaultEditorToolsVisibility: true,
        })
      );
    </script>
  </body>
</html>"#) } # [test] fn test_with_all_options () { use crate :: http :: graphiql_plugin_explorer ; let graphiql_source = GraphiQLSource :: build () . endpoint ("/") . subscription_endpoint ("/ws") . header ("Authorization" , "Bearer [token]") . version ("3.9.0") . ws_connection_param ("token" , "[token]") . title ("Awesome GraphiQL IDE Test") . credentials (Credentials :: Include) . plugins (& [graphiql_plugin_explorer ()]) . finish () ; assert_eq ! (graphiql_source , r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="robots" content="noindex">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="referrer" content="origin">

    <title>Awesome GraphiQL IDE Test</title>

    <style>
      body {
        height: 100%;
        margin: 0;
        width: 100%;
        overflow: hidden;
      }

      #graphiql {
        height: 100vh;
      }
    </style>
    <script
      crossorigin
      src="https://unpkg.com/react@18/umd/react.development.js"
    ></script>
    <script
      crossorigin
      src="https://unpkg.com/react-dom@18/umd/react-dom.development.js"
    ></script>
    <link rel="icon" href="https://graphql.org/favicon.ico">
    <link rel="stylesheet" href="https://unpkg.com/graphiql@3.9.0/graphiql.min.css" />
    <link rel="stylesheet" href="https://unpkg.com/@graphiql/plugin-explorer/dist/style.css" />
  </head>

  <body>
    <div id="graphiql">Loading...</div>
    <script
      src="https://unpkg.com/graphiql@3.9.0/graphiql.min.js"
      type="application/javascript"
    ></script>
    <script
      src="https://unpkg.com/@graphiql/plugin-explorer/dist/index.umd.js"
      crossorigin
    ></script>
    <script>
      customFetch = (url, opts = {}) => {
        return fetch(url, {...opts, credentials: 'include'})
      }

      createUrl = (endpoint, subscription = false) => {
        const url = new URL(endpoint, window.location.origin);
        if (subscription) {
          url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
        }
        return url.toString();
      }

      const plugins = [];
      plugins.push(GraphiQLPluginExplorer.explorerPlugin());

      ReactDOM.createRoot(document.getElementById("graphiql")).render(
        React.createElement(GraphiQL, {
          fetcher: GraphiQL.createFetcher({
            url: createUrl('/'),
            fetch: customFetch,
            subscriptionUrl: createUrl('/ws', true),
            headers: {
              'Authorization': 'Bearer [token]',
            },
            wsConnectionParams: {
              'token': '[token]',
            },
          }),
          defaultEditorToolsVisibility: true,
          plugins,
        })
      );
    </script>
  </body>
</html>"#) } }
};
}
